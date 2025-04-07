# OSM Tag Processing

## Overview of OSM Tags

OpenStreetMap (OSM) data uses tags (key-value pairs) to describe the attributes of map features. These tags are crucial for determining how roads should be represented in the routing graph. This chapter explores how Mjolnir processes OSM tags to extract relevant information for routing.

## Tag Structure

OSM tags consist of a key and a value, both of which are strings:

```
key=value
```

Common examples include:
- `highway=residential` (A residential road)
- `oneway=yes` (A one-way street)
- `maxspeed=50` (Speed limit of 50 km/h)
- `access=private` (Private access only)

## Tag Processing in Mjolnir

Mjolnir processes tags in several stages:

1. **Extraction**: Reading tags from OSM data
2. **Filtering**: Keeping only relevant tags
3. **Transformation**: Converting tags to internal representations
4. **Application**: Applying tag information to graph elements

### Tag Extraction

Tags are extracted during the initial parsing of OSM data:

```cpp
// From src/mjolnir/pbfgraphparser.cc
std::vector<Tag> PBFGraphParser::GetTagsFromWay(const OSMPBF::PrimitiveBlock& primblock,
                                             const OSMPBF::Way& way) {
  std::vector<Tag> tags;
  
  // For each tag in the way
  for (int i = 0; i < way.keys_size(); i++) {
    // Get the key and value strings
    std::string key = GetString(primblock, way.keys(i));
    std::string value = GetString(primblock, way.vals(i));
    
    // Add to the tags vector
    tags.emplace_back(key, value);
  }
  
  return tags;
}
```

### Tag Filtering

Not all OSM tags are relevant for routing. Mjolnir filters out irrelevant tags:

```cpp
// From src/mjolnir/pbfgraphparser.cc
bool PBFGraphParser::IsWayUsable(const std::vector<Tag>& tags) {
  // Check if this way is a road
  auto highway_tag = std::find_if(tags.begin(), tags.end(),
                                [](const Tag& tag) { return tag.first == "highway"; });
  
  // If there's no highway tag, it's not a road
  if (highway_tag == tags.end()) {
    return false;
  }
  
  // Check if it's a usable highway type
  const std::string& highway_value = highway_tag->second;
  return IsUsableHighway(highway_value);
}

bool PBFGraphParser::IsUsableHighway(const std::string& highway) {
  return highway == "motorway" || highway == "trunk" || highway == "primary" ||
         highway == "secondary" || highway == "tertiary" || highway == "unclassified" ||
         highway == "residential" || highway == "service" || highway == "motorway_link" ||
         highway == "trunk_link" || highway == "primary_link" || highway == "secondary_link" ||
         highway == "tertiary_link" || highway == "living_street" || highway == "pedestrian" ||
         highway == "footway" || highway == "cycleway" || highway == "path" ||
         highway == "steps" || highway == "track";
}
```

### Tag Transformation with Lua

One of Mjolnir's powerful features is its ability to use Lua scripts for tag transformation. This allows for customizable tag processing without modifying the C++ code:

```cpp
// From valhalla/mjolnir/luatagtransform.h
class LuaTagTransform {
public:
  LuaTagTransform(const std::string& lua_script);
  ~LuaTagTransform();
  
  // Transform tags for ways
  bool Transform(OSMWay& way, const Tags& tags);
  
  // Transform tags for nodes
  bool Transform(OSMNode& node, const Tags& tags);
  
private:
  lua_State* lua_state_;
};
```

The implementation loads and executes a Lua script:

```cpp
// From src/mjolnir/luatagtransform.cc
LuaTagTransform::LuaTagTransform(const std::string& lua_script) {
  // Create a new Lua state
  lua_state_ = luaL_newstate();
  luaL_openlibs(lua_state_);
  
  // Load the Lua script
  if (luaL_dofile(lua_state_, lua_script.c_str()) != 0) {
    throw std::runtime_error("Failed to load Lua script: " + lua_script);
  }
}

bool LuaTagTransform::Transform(OSMWay& way, const Tags& tags) {
  // Get the transform_way function from Lua
  lua_getglobal(lua_state_, "transform_way");
  
  // Create a table for the tags
  lua_newtable(lua_state_);
  for (const auto& tag : tags) {
    lua_pushstring(lua_state_, tag.second.c_str());
    lua_setfield(lua_state_, -2, tag.first.c_str());
  }
  
  // Call the function
  if (lua_pcall(lua_state_, 1, 1, 0) != 0) {
    throw std::runtime_error("Failed to call transform_way: " + 
                           std::string(lua_tostring(lua_state_, -1)));
  }
  
  // Check if the way should be included
  if (!lua_isnil(lua_state_, -1) && lua_istable(lua_state_, -1)) {
    // Process the result table and update the way
    UpdateWayFromTable(way);
    lua_pop(lua_state_, 1);
    return true;
  }
  
  // Way should be skipped
  lua_pop(lua_state_, 1);
  return false;
}
```

### Default Lua Script

Valhalla includes a default Lua script for tag processing. Here's a simplified example:

```lua
-- Function to process way tags
function transform_way(tags)
  -- Check if this is a road
  local highway = tags["highway"]
  if not highway then
    return nil
  end
  
  -- Initialize the result
  local result = {}
  
  -- Set the road class
  result.road_class = road_class_mapping[highway] or "service"
  
  -- Check if it's one-way
  result.oneway = tags["oneway"] == "yes"
  
  -- Get the speed limit
  local maxspeed = tags["maxspeed"]
  if maxspeed then
    result.speed = parse_speed(maxspeed)
  else
    -- Use default speed based on road class
    result.speed = default_speeds[result.road_class]
  end
  
  -- Check access restrictions
  result.access = process_access_tags(tags)
  
  return result
end

-- Function to process node tags
function transform_node(tags)
  -- Check if this is an intersection
  local junction = tags["junction"]
  if junction then
    return {junction_type = junction}
  end
  
  -- Check if this is a traffic signal
  if tags["highway"] == "traffic_signals" then
    return {traffic_signal = true}
  end
  
  -- Check if this is a stop sign
  if tags["highway"] == "stop" then
    return {stop_sign = true}
  end
  
  return nil
end
```

## Key Tag Categories

Mjolnir processes several categories of tags:

### 1. Road Classification

Road classification determines the hierarchy level and importance of roads:

```cpp
// From src/mjolnir/osmway.cc
RoadClass OSMWay::GetRoadClass(const std::string& highway) {
  if (highway == "motorway")        return RoadClass::kMotorway;
  else if (highway == "trunk")      return RoadClass::kTrunk;
  else if (highway == "primary")    return RoadClass::kPrimary;
  else if (highway == "secondary")  return RoadClass::kSecondary;
  else if (highway == "tertiary")   return RoadClass::kTertiary;
  else if (highway == "unclassified") return RoadClass::kUnclassified;
  else if (highway == "residential") return RoadClass::kResidential;
  else if (highway == "service")    return RoadClass::kService;
  else                              return RoadClass::kServiceOther;
}
```

### 2. Access Restrictions

Access restrictions determine which modes of travel can use a road:

```cpp
// From src/mjolnir/osmway.cc
void OSMWay::SetAccess(const Tags& tags) {
  // Default access
  access_ = kAllAccess;
  
  // Check for access restrictions
  auto access_tag = std::find_if(tags.begin(), tags.end(),
                               [](const Tag& tag) { return tag.first == "access"; });
  if (access_tag != tags.end()) {
    if (access_tag->second == "private" || access_tag->second == "no") {
      access_ = 0;
    }
  }
  
  // Mode-specific access
  auto car_tag = std::find_if(tags.begin(), tags.end(),
                            [](const Tag& tag) { return tag.first == "motor_vehicle"; });
  if (car_tag != tags.end()) {
    if (car_tag->second == "no") {
      access_ &= ~kAutoAccess;
    }
  }
  
  // ... check other modes ...
}
```

### 3. Speed Limits

Speed limits determine how fast vehicles can travel on a road:

```cpp
// From src/mjolnir/osmway.cc
uint32_t OSMWay::ParseSpeed(const std::string& maxspeed) {
  // Check for special values
  if (maxspeed == "walk" || maxspeed == "walking") {
    return 5;  // 5 km/h
  } else if (maxspeed == "none" || maxspeed == "unlimited") {
    return kUnlimitedSpeed;
  }
  
  // Parse the numeric value
  std::string value;
  std::string units;
  SplitSpeedValue(maxspeed, value, units);
  
  // Convert to km/h
  float speed = std::stof(value);
  if (units == "mph") {
    speed = speed * 1.609344f;
  }
  
  return static_cast<uint32_t>(speed);
}
```

### 4. Surface Type

Surface type affects travel speed and suitability for different modes:

```cpp
// From src/mjolnir/osmway.cc
Surface OSMWay::GetSurface(const std::string& surface) {
  if (surface == "paved" || surface == "asphalt" || surface == "concrete") {
    return Surface::kPaved;
  } else if (surface == "unpaved" || surface == "gravel") {
    return Surface::kGravel;
  } else if (surface == "dirt" || surface == "earth" || surface == "ground") {
    return Surface::kDirt;
  } else if (surface == "grass") {
    return Surface::kGrass;
  } else if (surface == "sand") {
    return Surface::kSand;
  } else if (surface == "wood") {
    return Surface::kWood;
  } else if (surface == "metal") {
    return Surface::kMetal;
  } else if (surface == "compacted") {
    return Surface::kCompacted;
  } else {
    return Surface::kPaved;  // Default to paved
  }
}
```

### 5. Direction of Travel

Direction of travel determines if a road is one-way or bidirectional:

```cpp
// From src/mjolnir/osmway.cc
bool OSMWay::IsOneWay(const Tags& tags) {
  // Check for oneway tag
  auto oneway_tag = std::find_if(tags.begin(), tags.end(),
                               [](const Tag& tag) { return tag.first == "oneway"; });
  if (oneway_tag != tags.end()) {
    return oneway_tag->second == "yes" || oneway_tag->second == "1" || oneway_tag->second == "true";
  }
  
  // Check for junction=roundabout (implied oneway)
  auto junction_tag = std::find_if(tags.begin(), tags.end(),
                                 [](const Tag& tag) { return tag.first == "junction"; });
  if (junction_tag != tags.end()) {
    return junction_tag->second == "roundabout";
  }
  
  // Check for highway=motorway (implied oneway)
  auto highway_tag = std::find_if(tags.begin(), tags.end(),
                                [](const Tag& tag) { return tag.first == "highway"; });
  if (highway_tag != tags.end()) {
    return highway_tag->second == "motorway";
  }
  
  return false;
}
```

### 6. Special Road Types

Special road types require specific handling:

```cpp
// From src/mjolnir/osmway.cc
bool OSMWay::IsRoundabout(const Tags& tags) {
  // Check for junction=roundabout
  auto junction_tag = std::find_if(tags.begin(), tags.end(),
                                 [](const Tag& tag) { return tag.first == "junction"; });
  if (junction_tag != tags.end()) {
    return junction_tag->second == "roundabout";
  }
  
  return false;
}

bool OSMWay::IsBridge(const Tags& tags) {
  // Check for bridge tag
  auto bridge_tag = std::find_if(tags.begin(), tags.end(),
                               [](const Tag& tag) { return tag.first == "bridge"; });
  if (bridge_tag != tags.end()) {
    return bridge_tag->second == "yes" || bridge_tag->second == "1" || bridge_tag->second == "true";
  }
  
  return false;
}

bool OSMWay::IsTunnel(const Tags& tags) {
  // Check for tunnel tag
  auto tunnel_tag = std::find_if(tags.begin(), tags.end(),
                               [](const Tag& tag) { return tag.first == "tunnel"; });
  if (tunnel_tag != tags.end()) {
    return tunnel_tag->second == "yes" || tunnel_tag->second == "1" || tunnel_tag->second == "true";
  }
  
  return false;
}
```

## Tag Processing Flow

The complete tag processing flow in Mjolnir follows these steps:

1. **Extract Tags**: Read tags from OSM data
2. **Filter Tags**: Keep only relevant tags
3. **Transform Tags**: Apply Lua transformation
4. **Apply Tags**: Update internal data structures with tag information
5. **Validate Tags**: Check for consistency and completeness

```
+-------------------+     +-------------------+     +-------------------+
| Extract Tags      |     | Filter Tags       |     | Transform Tags    |
| from OSM          | --> | (Keep Relevant)   | --> | (Apply Lua)       |
+-------------------+     +-------------------+     +-------------------+
                                                           |
                                                           v
+-------------------+     +-------------------+
| Validate Tags     |     | Apply Tags        |
| (Check Consistency)| <-- | (Update Data)     |
+-------------------+     +-------------------+
```

## Customizing Tag Processing

One of the strengths of Mjolnir is the ability to customize tag processing through Lua scripts. This allows for:

1. **Regional Customizations**: Different regions may have different tagging conventions
2. **Special Use Cases**: Custom routing for specific purposes (e.g., emergency vehicles)
3. **Data Corrections**: Fixing common tagging errors or inconsistencies

To customize tag processing, create a custom Lua script and specify it in the configuration:

```json
{
  "mjolnir": {
    "lua_script": "/path/to/custom_tag_transform.lua"
  }
}
```

## Common Tag Processing Challenges

Tag processing presents several challenges:

### 1. Inconsistent Tagging

OSM data is contributed by many different mappers, leading to inconsistencies:

```cpp
// Example of handling inconsistent tagging
std::string NormalizeHighwayTag(const std::string& highway) {
  // Convert to lowercase
  std::string normalized = highway;
  std::transform(normalized.begin(), normalized.end(), normalized.begin(), ::tolower);
  
  // Handle common variations
  if (normalized == "motorway_link") return "motorway_link";
  if (normalized == "motorwaylink") return "motorway_link";
  if (normalized == "motor_way_link") return "motorway_link";
  
  return normalized;
}
```

### 2. Missing Tags

Some important tags may be missing:

```cpp
// Example of handling missing tags
void InferMissingTags(OSMWay& way, const Tags& tags) {
  // If speed is missing, infer from road class
  if (way.speed() == 0) {
    way.set_speed(GetDefaultSpeed(way.road_class()));
  }
  
  // If surface is missing, infer from road class
  if (way.surface() == Surface::kUnknown) {
    way.set_surface(GetDefaultSurface(way.road_class()));
  }
}
```

### 3. Conflicting Tags

Tags may contradict each other:

```cpp
// Example of handling conflicting tags
void ResolveConflictingTags(OSMWay& way, const Tags& tags) {
  // If a road is both a footway and a cycleway, prioritize cycleway
  auto highway_tag = std::find_if(tags.begin(), tags.end(),
                                [](const Tag& tag) { return tag.first == "highway"; });
  auto bicycle_tag = std::find_if(tags.begin(), tags.end(),
                                [](const Tag& tag) { return tag.first == "bicycle"; });
  
  if (highway_tag->second == "footway" && bicycle_tag != tags.end() && bicycle_tag->second == "designated") {
    way.set_use(Use::kCycleway);
  }
}
```

## Understanding OSM tag processing is crucial for building a graph tile builder that accurately represents the road network. The tag information determines how roads are classified, which modes of travel can use them, and how they should be traversed during routing.
