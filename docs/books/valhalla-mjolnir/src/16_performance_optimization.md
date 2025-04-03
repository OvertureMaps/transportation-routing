# Performance Optimization

## The Importance of Performance

Building graph tiles for planet-scale data is computationally intensive. This chapter explores the performance optimization techniques used in Mjolnir to efficiently process large datasets.

## Memory Management

Efficient memory management is crucial for processing large datasets:

### Memory-Mapped Files

Mjolnir uses memory-mapped files for efficient access to large datasets:

```cpp
// From src/mjolnir/pbfgraphparser.cc
void PBFGraphParser::Parse(const std::string& filename) {
  // Memory map the file
  boost::iostreams::mapped_file_source file;
  file.open(filename);
  
  if (!file.is_open()) {
    throw std::runtime_error("Failed to open file: " + filename);
  }
  
  // Process the data in chunks
  const char* data = file.data();
  const size_t size = file.size();
  
  // ... process data ...
  
  // File is automatically unmapped when file goes out of scope
}
```

### Custom Memory Pools

For allocating many small objects efficiently, Mjolnir uses custom memory pools:

```cpp
// From midgard/memorypool.h
template <typename T>
class MemoryPool {
public:
  MemoryPool(size_t block_size = 1024);
  ~MemoryPool();
  
  // Allocate an object
  T* Allocate();
  
  // Free an object
  void Free(T* ptr);
  
private:
  // Allocate a new block
  void AllocateBlock();
  
  // List of allocated blocks
  std::vector<T*> blocks_;
  
  // Free list
  T* free_list_;
  
  // Block size
  size_t block_size_;
};
```

The implementation allocates memory in large blocks and manages a free list:

```cpp
// From midgard/memorypool.h
template <typename T>
T* MemoryPool<T>::Allocate() {
  // If the free list is empty, allocate a new block
  if (free_list_ == nullptr) {
    AllocateBlock();
  }
  
  // Get an object from the free list
  T* result = free_list_;
  free_list_ = *reinterpret_cast<T**>(free_list_);
  
  // Construct the object
  new (result) T();
  
  return result;
}

template <typename T>
void MemoryPool<T>::Free(T* ptr) {
  // Destruct the object
  ptr->~T();
  
  // Add to the free list
  *reinterpret_cast<T**>(ptr) = free_list_;
  free_list_ = ptr;
}

template <typename T>
void MemoryPool<T>::AllocateBlock() {
  // Allocate a new block
  T* block = static_cast<T*>(::operator new(sizeof(T) * block_size_));
  blocks_.push_back(block);
  
  // Initialize the free list
  free_list_ = block;
  
  // Link the objects in the free list
  for (size_t i = 0; i < block_size_ - 1; i++) {
    *reinterpret_cast<T**>(block + i) = block + i + 1;
  }
  
  // Terminate the free list
  *reinterpret_cast<T**>(block + block_size_ - 1) = nullptr;
}
```

### Streaming Processing

To avoid loading the entire dataset into memory, Mjolnir processes data in a streaming fashion:

```cpp
// From src/mjolnir/pbfgraphparser.cc
void PBFGraphParser::Parse(const std::vector<std::string>& input_files) {
  // For each input file
  for (const auto& file : input_files) {
    // Open the file
    std::ifstream input(file, std::ios::binary);
    
    // Process the file in chunks
    const size_t chunk_size = 1024 * 1024;  // 1 MB
    std::vector<char> buffer(chunk_size);
    
    while (input) {
      // Read a chunk
      input.read(buffer.data(), chunk_size);
      size_t bytes_read = input.gcount();
      
      if (bytes_read == 0) {
        break;
      }
      
      // Process the chunk
      ProcessChunk(buffer.data(), bytes_read);
    }
  }
}
```

## Parallel Processing

Mjolnir uses parallel processing to take advantage of multiple CPU cores:

### Thread Pools

A thread pool is used to execute tasks in parallel:

```cpp
// From midgard/threadpool.h
class ThreadPool {
public:
  ThreadPool(size_t num_threads = std::thread::hardware_concurrency());
  ~ThreadPool();
  
  // Add a task to the pool
  template<typename F, typename... Args>
  auto Enqueue(F&& f, Args&&... args) -> std::future<decltype(f(args...))>;
  
private:
  // Worker threads
  std::vector<std::thread> workers_;
  
  // Task queue
  std::queue<std::function<void()>> tasks_;
  
  // Synchronization
  std::mutex queue_mutex_;
  std::condition_variable condition_;
  bool stop_;
};
```

The implementation creates a pool of worker threads that process tasks from a queue:

```cpp
// From midgard/threadpool.h
ThreadPool::ThreadPool(size_t num_threads) : stop_(false) {
  for (size_t i = 0; i < num_threads; i++) {
    workers_.emplace_back([this] {
      while (true) {
        std::function<void()> task;
        
        {
          std::unique_lock<std::mutex> lock(queue_mutex_);
          condition_.wait(lock, [this] { return stop_ || !tasks_.empty(); });
          
          if (stop_ && tasks_.empty()) {
            return;
          }
          
          task = std::move(tasks_.front());
          tasks_.pop();
        }
        
        task();
      }
    });
  }
}

template<typename F, typename... Args>
auto ThreadPool::Enqueue(F&& f, Args&&... args) -> std::future<decltype(f(args...))> {
  using return_type = decltype(f(args...));
  
  auto task = std::make_shared<std::packaged_task<return_type()>>(
    std::bind(std::forward<F>(f), std::forward<Args>(args)...)
  );
  
  std::future<return_type> result = task->get_future();
  
  {
    std::unique_lock<std::mutex> lock(queue_mutex_);
    
    if (stop_) {
      throw std::runtime_error("Enqueue on stopped ThreadPool");
    }
    
    tasks_.emplace([task]() { (*task)(); });
  }
  
  condition_.notify_one();
  return result;
}
```

### Parallel Tile Processing

Tiles can be processed in parallel since they're largely independent:

```cpp
// From src/mjolnir/graphbuilder.cc
void GraphBuilder::BuildTiles() {
  // Get the list of tiles to build
  std::vector<GraphId> tile_ids = GetTileList();
  
  // Create a thread pool
  ThreadPool pool;
  
  // Process tiles in parallel
  std::vector<std::future<void>> results;
  for (const auto& tile_id : tile_ids) {
    results.emplace_back(pool.Enqueue([this, tile_id]() {
      BuildTile(tile_id);
    }));
  }
  
  // Wait for all tiles to be processed
  for (auto& result : results) {
    result.get();
  }
}
```

### OpenMP Integration

For simpler parallel processing, Mjolnir uses OpenMP:

```cpp
// From src/mjolnir/graphenhancer.cc
void GraphEnhancer::Enhance() {
  // Get the list of tiles to enhance
  std::vector<GraphId> tile_ids = GetTileList();
  
  // Process tiles in parallel using OpenMP
  #pragma omp parallel for
  for (size_t i = 0; i < tile_ids.size(); i++) {
    EnhanceTile(tile_ids[i]);
  }
}
```

## Algorithmic Optimizations

Mjolnir uses various algorithmic optimizations to improve performance:

### Spatial Indexing

For efficient spatial queries, Mjolnir uses spatial indexes:

```cpp
// From midgard/rtree.h
template <typename T>
class RTree {
public:
  RTree();
  
  // Insert an item
  void Insert(const T& item, const BoundingBox& bounds);
  
  // Query items within a bounding box
  std::vector<T> Query(const BoundingBox& bounds) const;
  
  // Nearest neighbor query
  std::vector<T> Nearest(const Point& point, size_t count) const;
  
private:
  // Internal implementation
  struct Node;
  std::unique_ptr<Node> root_;
};
```

The implementation uses an R-tree data structure for efficient spatial indexing:

```cpp
// From midgard/rtree.h
template <typename T>
void RTree<T>::Insert(const T& item, const BoundingBox& bounds) {
  if (!root_) {
    root_ = std::make_unique<Node>(bounds);
    root_->items.push_back(item);
    return;
  }
  
  // Find the best leaf node for insertion
  Node* leaf = FindBestLeaf(root_.get(), bounds);
  
  // Insert the item
  leaf->items.push_back(item);
  leaf->bounds.Expand(bounds);
  
  // Rebalance the tree if needed
  if (leaf->items.size() > max_items_) {
    SplitNode(leaf);
  }
}

template <typename T>
std::vector<T> RTree<T>::Query(const BoundingBox& bounds) const {
  std::vector<T> results;
  
  if (!root_) {
    return results;
  }
  
  // Recursive query
  QueryNode(root_.get(), bounds, results);
  
  return results;
}
```

### Hash Tables

For fast lookups, Mjolnir uses hash tables:

```cpp
// From src/mjolnir/osmdata.cc
void OSMData::BuildNodeMap() {
  // Create a hash map for node lookups
  node_map_.reserve(nodes.size());
  
  for (const auto& node : nodes) {
    node_map_[node.id] = &node;
  }
}

const OSMNode* OSMData::GetNode(uint64_t id) const {
  auto it = node_map_.find(id);
  if (it != node_map_.end()) {
    return it->second;
  }
  return nullptr;
}
```

### String Interning

To reduce memory usage for repeated strings, Mjolnir uses string interning:

```cpp
// From src/mjolnir/graphtilebuilder.cc
uint32_t GraphTileBuilder::AddName(const std::string& name) {
  // Check if the name already exists in the text list
  auto it = text_offset_map_.find(name);
  if (it != text_offset_map_.end()) {
    return it->second;
  }
  
  // Add the name to the text list
  uint32_t offset = textlist_.size();
  textlist_.append(name);
  textlist_.push_back('\0');  // Null terminator
  
  // Add to the offset map
  text_offset_map_[name] = offset;
  
  return offset;
}
```

## Data Structure Optimizations

Mjolnir uses optimized data structures to improve performance:

### Bit Packing

Multiple fields are packed into a single integer to save space:

```cpp
// From baldr/nodeinfo.h
class NodeInfo {
public:
  // Set methods
  void set_latlng(const std::pair<float, float>& ll);
  void set_access(const uint32_t access);
  void set_type(const NodeType type);
  
private:
  uint64_t field1_;      // Lat,lng packed as uint64_t
  uint32_t field2_;      // Access, intersection type, admin index
  uint32_t field3_;      // Edge index, edge count, time zone
};
```

The implementation packs multiple fields into each integer:

```cpp
// From baldr/nodeinfo.cc
void NodeInfo::set_latlng(const std::pair<float, float>& ll) {
  // Convert to fixed-point and pack into field1_
  uint32_t lat = LatLng::Float2Fixed(ll.first);
  uint32_t lng = LatLng::Float2Fixed(ll.second);
  field1_ = (static_cast<uint64_t>(lat) << 32) | static_cast<uint64_t>(lng);
}

void NodeInfo::set_access(const uint32_t access) {
  // Pack into field2_
  field2_ = (field2_ & ~kAccessMask) | (access & kAccessMask);
}

void NodeInfo::set_type(const NodeType type) {
  // Pack into field2_
  field2_ = (field2_ & ~kTypeShift) | (static_cast<uint32_t>(type) << kTypeShift);
}
```

### Cache-Friendly Data Structures

To improve cache locality, Mjolnir uses cache-friendly data structures:

```cpp
// From baldr/graphtile.h
class GraphTile {
public:
  GraphTile(const GraphId& id, char* ptr, size_t size);
  
  // Header information
  const GraphTileHeader* header() const;
  
  // Access to nodes
  const NodeInfo* node(const uint32_t id) const;
  
  // Access to directed edges
  const DirectedEdge* directededge(const uint32_t idx) const;
  
private:
  GraphId graphid_;        // Tile ID
  size_t size_;            // Size in bytes
  char* graphtile_;        // Pointer to memory
  char* header_;           // Pointer to the header
  char* nodes_;            // Pointer to nodes
  char* directededges_;    // Pointer to directed edges
  // ... pointers to other sections ...
};
```

The implementation stores related data together for better cache locality:

```cpp
// From baldr/graphtile.cc
GraphTile::GraphTile(const GraphId& id, char* ptr, size_t size)
    : graphid_(id), size_(size), graphtile_(ptr) {
  // Set pointers to the start of each section
  header_ = graphtile_;
  nodes_ = graphtile_ + sizeof(GraphTileHeader);
  directededges_ = nodes_ + header_->nodecount() * sizeof(NodeInfo);
  // ... set other pointers ...
}

const NodeInfo* GraphTile::node(const uint32_t id) const {
  if (id < header_->nodecount()) {
    return &nodes_[id];
  }
  throw std::runtime_error("NodeInfo index out of bounds: " + std::to_string(id));
}

const DirectedEdge* GraphTile::directededge(const uint32_t idx) const {
  if (idx < header_->directededgecount()) {
    return &directededges_[idx];
  }
  throw std::runtime_error("DirectedEdge index out of bounds: " + std::to_string(idx));
}
```

## I/O Optimizations

Efficient I/O is crucial for processing large datasets:

### Buffered I/O

Mjolnir uses buffered I/O for efficient file access:

```cpp
// From src/mjolnir/graphtilebuilder.cc
void GraphTileBuilder::StoreTileData() {
  // Create the file name
  std::string filename = tile_dir_ + "/" + GraphTile::FileSuffix(header_->graphid());
  
  // Make sure the directory exists
  boost::filesystem::create_directories(boost::filesystem::path(filename).parent_path());
  
  // Open the file with a buffer
  std::ofstream file(filename, std::ios::out | std::ios::binary);
  if (!file.is_open()) {
    throw std::runtime_error("Failed to open file: " + filename);
  }
  
  // Set a large buffer size
  std::vector<char> buffer(1024 * 1024);  // 1 MB buffer
  file.rdbuf()->pubsetbuf(buffer.data(), buffer.size());
  
  // Write the header
  file.write(reinterpret_cast<const char*>(header_), sizeof(GraphTileHeader));
  
  // Write the nodes
  file.write(reinterpret_cast<const char*>(nodes_.data()), nodes_.size() * sizeof(NodeInfo));
  
  // Write the directed edges
  file.write(reinterpret_cast<const char*>(directededges_.data()),
             directededges_.size() * sizeof(DirectedEdge));
  
  // ... write other sections ...
  
  // Close the file
  file.close();
}
```

### Batch Processing

To reduce I/O overhead, Mjolnir processes data in batches:

```cpp
// From src/mjolnir/graphbuilder.cc
void GraphBuilder::BuildTiles() {
  // Group nodes by tile
  std::unordered_map<GraphId, std::vector<OSMNode>> nodes_by_tile;
  for (const auto& node : osmdata_.nodes) {
    GraphId tile_id = GetTileId(node);
    nodes_by_tile[tile_id].push_back(node);
  }
  
  // Group ways by tile
  std::unordered_map<GraphId, std::vector<OSMWay>> ways_by_tile;
  for (const auto& way : osmdata_.ways) {
    // A way can span multiple tiles
    std::unordered_set<GraphId> tiles;
    for (const auto& node_id : way.node_ids) {
      const auto& node = osmdata_.node_map.at(node_id);
      GraphId tile_id = GetTileId(node);
      tiles.insert(tile_id);
    }
    
    // Add the way to each tile it touches
    for (const auto& tile_id : tiles) {
      ways_by_tile[tile_id].push_back(way);
    }
  }
  
  // Process each tile
  for (const auto& [tile_id, nodes] : nodes_by_tile) {
    const auto& ways = ways_by_tile[tile_id];
    BuildTile(tile_id, nodes, ways);
  }
}
```

## Profiling and Benchmarking

Mjolnir includes tools for profiling and benchmarking to identify performance bottlenecks:

```cpp
// From src/mjolnir/valhalla_build_tiles.cc
int main(int argc, char** argv) {
  // Parse command line arguments
  // ...
  
  // Start timing
  auto start_time = std::chrono::high_resolution_clock::now();
  
  // Build the tiles
  GraphBuilder builder;
  builder.Build(config, input_files);
  
  // End timing
  auto end_time = std::chrono::high_resolution_clock::now();
  auto duration = std::chrono::duration_cast<std::chrono::seconds>(end_time - start_time);
  
  // Report timing
  std::cout << "Tile building completed in " << duration.count() << " seconds" << std::endl;
  
  return 0;
}
```

## Performance Metrics

Mjolnir tracks various performance metrics:

```cpp
// From src/mjolnir/graphbuilder.cc
void GraphBuilder::Build(const Config& config, const std::vector<std::string>& input_files) {
  // Initialize metrics
  size_t node_count = 0;
  size_t edge_count = 0;
  size_t tile_count = 0;
  
  // Parse input data
  auto parse_start = std::chrono::high_resolution_clock::now();
  parser_.Parse(input_files);
  auto parse_end = std::chrono::high_resolution_clock::now();
  auto parse_duration = std::chrono::duration_cast<std::chrono::seconds>(parse_end - parse_start);
  
  // Build the tiles
  auto build_start = std::chrono::high_resolution_clock::now();
  BuildTiles();
  auto build_end = std::chrono::high_resolution_clock::now();
  auto build_duration = std::chrono::duration_cast<std::chrono::seconds>(build_end - build_start);
  
  // Collect metrics
  for (const auto& tile : tiles_) {
    node_count += tile.node_count;
    edge_count += tile.edge_count;
    tile_count++;
  }
  
  // Report metrics
  std::cout << "Parsing completed in " << parse_duration.count() << " seconds" << std::endl;
  std::cout << "Tile building completed in " << build_duration.count() << " seconds" << std::endl;
  std::cout << "Created " << tile_count << " tiles with " << node_count << " nodes and "
            << edge_count << " edges" << std::endl;
}
```

## Performance optimization is crucial for building a graph tile builder that can handle planet-scale data. By using efficient memory management, parallel processing, algorithmic optimizations, and optimized data structures, Mjolnir can process large datasets in a reasonable amount of time.
