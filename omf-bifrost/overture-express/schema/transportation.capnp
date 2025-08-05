@0xf1e2d3c4b5a69788;

# Basic transportation elements for Overture Maps
struct Coordinate {
  lon @0 :Float64;
  lat @1 :Float64;
}

struct Point {
  coordinate @0 :Coordinate;
}

struct LineString {
  coordinates @0 :List(Coordinate);
}

enum RoadClass {
  motorway @0;
  trunk @1;
  primary @2;
  secondary @3;
  tertiary @4;
  residential @5;
  service @6;
  unclassified @7;
}

struct Segment {
  id @0 :Text;
  geometry @1 :LineString;
  class @2 :RoadClass;
  subtype @3 :Text;
  connectors @4 :List(Text);  # List of connector IDs
}

struct Connector {
  id @0 :Text;
  geometry @1 :Point;
  connectedSegments @2 :List(Text);  # List of segment IDs
}
