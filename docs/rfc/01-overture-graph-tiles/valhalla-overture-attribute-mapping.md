# Bifrost Attribute Mapping   
- Author: Rob Haley (rhaley@esri.com)
- Status: Draft
- Created: 2025-06-09
- Last Updated: 2025-06-27
## Road Classification   
- Valhalla uses road classes for applying routing rules and assigning hierarchy for search. Overture has a class structure that needs to be mapped to the Valhalla road classes. The below table represents a mapping methodology until additional road classes are available.
  
	| Overture Class | Valhalla Road Class |
	|:---------------|:--------------------|
	|       motorway |           kMotorway |
	|          trunk |              kTrunk |
	|        primary |            kPrimary |
	|      secondary |          kSecondary |
	|       tertiary |           kTertiary |
	|    residential |        kResidential |
	|   unclassified |       kUnclassified |
	|        service |     kService\_Other |
	|     pedestrian |     kService\_Other |
	|        footway |     kService\_Other |
	|          alley |     kService\_Other |
	|      crosswalk |     kService\_Other |
	|       cycleway |     kService\_Other |
	|       driveway |     kService\_Other |
	| living\_street |     kService\_Other |
	| parking\_aisle |     kService\_Other |
	|           path |     kService\_Other |
	|       sidewalk |     kService\_Other |
	|          steps |     kService\_Other |
	|          track |     kService\_Other |
	|        unknown |       kUnclassified |

## Access Restrictions   
- Access restrictions in Overture currently have a lot of needless complexity. Roads can get one of three assignment types: designated, allowed or denied. Those assignments then get a mode of travel assigned to them. A better long term approach would be to revisit the scheme approach in Overture to utilize only the denied assignment and assume all modes of transit are available for any road absent that assignment. That would greatly simplify the structure and reduce all the opportunities for conflict.   
- Currently within Overture there is an innumerable amount combinations of assignments that could come in for a specific extent. A cost of this is that in order to know all the assignments that would need to be supported for a given extent, all the roads must be read in. A simpler approach and a future design clarification should be setting a fixed amount of assignment that should be supported, assigning default values when those are not present and ignoring unsupported values. That could be done in schema restriction or here although handling it on the data side has downstream benefits for other solutions.
- As it currently exists, identifying whether or not a vehicle has access will need to be done through a combination of factors and designated below:
    - kAutoAccess   
        - Is   
            - designated\_motor\_vehicle   
            - allowed\_vehicle   
            - allowed\_car   
            - allowed\_motor\_vehicle   
            - designated\_car   
        - Is not   
            - designated\_bicycle   
            - designated\_hgv   
            - designated\_foot   
            - designated\_bus   
            - denied\_motor\_vehicle   
            - denied\_car   
            - denied\_vehicle   
    - kBicycleAccess   
        - Is   
            - designated\_bicycle   
            - allowed\_bicycle   
        - Is not   
            - designated\_foot   
            - designated\_hgv   
            - designated\_motor\_vehicle   
            - designated\_bus   
            - designated\_car   
            - designated\_vehicle   
            - denied\_bicycle   
    - kBusAccess   
        - Is   
            - designated\_bus   
            - allowed\_bus   
        - Is not   
            - designated\_bicycle   
            - designated\_hgv   
            - designated\_foot   
            - designated\_car   
            - denied\_vehicle   
            - denied\_bus   
            - denied\_motor\_vehicle   
    - kTruckAccess   
        - Is   
            - designated\_hgv   
            - allowed\_hgv   
        - Is not   
            - designated\_bicycle   
            - designated\_foot   
            - designated\_vehicle   
            - designated\_car   
            - designated\_bus   
            - denied\_vehicle   
            - denied\_hgv   
            - denied\_motor\_vehicle   
    - kPedestrianAccess   
        - Is   
            - designated\_foot   
            - allowed\_foot   
        - Is not   
            - designated\_bicycle   
            - designated\_hgv   
            - designated\_motor\_vehicle   
            - designated\_bus   
            - designated\_car   
            - designated\_vehicle   
            - denied\_foot
    - Future
        - kBikeshareAccess   
        - kTaxiAccess   
        - kMotor\_scooterAccess   
        - kMotorcycleAccess
- Because of the opportunities for conflict, a teiring system needs to exist to break said conflicts. There would be a two teired logic system like follows:
    - designated > denied > allowed
    - foot > bicycle > bus > hgv > car > motor_vehicle
    - As laid out, all designated assignmentments would have first precedent. If there were two deginated assignments, then one example would be foot would get precedent over hgv. 
   
- Valhalla makes use of bidirectional directed edge restrictions by separating them into start\_restriction and end\_restriction. The general structure end up looking like this:

```
"edge": {
      "tunnel": false,

      "bridge": false,

      "access": {

        "truck": true,

        "pedestrian": true,

        "wheelchair": true,

        "taxi": true,

        "HOV": true,

        "emergency": false,

        "motorcycle": true,

        "car": true,

        "moped": true,

        "bus": true,

        "bicycle": true

      },

      "has_sign": false,

      "deadend": false,

      "country_crossing": false,

      "end_restriction": {

        "truck": false,

        "pedestrian": false,

        "wheelchair": false,

        "taxi": false,

        "HOV": false,

        "emergency": false,

        "motorcycle": false,

        "car": false,

        "moped": false,

        "bus": false,

        "bicycle": false

      },

      "cycle_lane": "none",
      "start_restriction": {

        "truck": false,

        "pedestrian": false,

        "wheelchair": false,

        "taxi": false,

        "HOV": false,

        "emergency": false,

        "motorcycle": false,

        "car": false,

        "moped": false,

        "bus": false,

        "bicycle": false

      },
```

    
- Overture designates travel rules with either a backwards or forwards designation in the when: heading: like so:   


```
    {
"access_type": "denied",
		"when": {
			"heading": "backward"
		}
	}
```
    
- For purposes of applying that to bifrost:    
    - Forwards designation will apply to start\_restriction   
    - Backwards designation will apply to end\_restriction   
## Speed Limits   
- The existing Valhalla approach to speed is likely viable for this solution. If there is a posted maxspeed attribute, it will be mapped to max_speed. When there is none and the road is classed as highway, then it uses an internal default highway speed. For all other roads it uses a density based approach to determine if it is a rural or urban road and classes a speed based on that.
- An alternative approach would be to create global averages for the dataset based on road type and fill in nulls based on the road type. This approach has shown to have some good application but rural/urban divides are noticable.
## Surface Types   
- Valhalla uses surface types as a parameter for restricting travel as requested. Below are the Valhalla surface types and the recommended match types from Overture.
  
	| Overture Class | Valhalla Road Class |
	|:---------------|:--------------------|
	|          metal |        paved_smooth |
	|          paved |               paved |
	|         bricks |         paved_rough |
	|  paving_stones |           compacted |
	|           dirt |                dirt |
	|         gravel |              gravel |
	|   unclassified |                path |
	|        service |          impassable |
	|        unknown |                path |
	|        unpaved |     		  dirt |
	|        asphalt |               paved |
	|    cobblestone |           compacted |
	|           wood |         paved_rough |
	|         rubber |        paved_smooth |
	|          tiles |           compacted |
	|         shells |              gravel |
	|           rock |              gravel |
	|     all others |                path |

## Direction of Travel   
- For any given edge, both directions of travel are assumed to be supported unless explicitly restricted for a given travel mode. 
## Vehicle Options   
- Valhalla supports the following restrictions for auto, bus, taxi and truck (not proposing to support taxi currently, atuo = motor_vehicle, truck = hgv and bus = bus in OVerture terms): 
    - height   
    - width
    - exclude_unpaved
    - exclude_cash_only_tolls
    - include_hov2
    - include_hov3
    - include_hot
- For trucks (hgv) Valhalla also supports:
    - length
    - weight
    - axle_load
    - axle_count
    - hazmat
    - hgv_no_access_penlty
    - low_class_penalty
    - use_truck_route
For mapping purposes:
    - in cases where a vehicle type may be specified as a part of the forwards or backwards dimensions/value designation, the vehicle type will be ignored
    - Valhalla used meters and metric tons by default. When a value is identified, the corresponding unit will need to be indentified and the units will need to be converted
    - where there is any forwards or backwards denied dim of weight, the corresponding value in val will get mapped to the weight
    - where there is any  forwards or backwards denied dim of height, the corresponding value in val will get mapped to height
    - where there is any  forwards or backwards denied dim of length, the corresponding value in val will get mapped to length
    - where there is any  forwards or backwards denied dim of axle_load, the corresponding value in val will get mapped to axle_load
    - where there is any  forwards or backwards denied dim of axle_count, the corresponding value in val will get mapped to axle_count
    - where exclude_unpaved, all roads with the tags other than paved, paved_smooth or paved_rough from surface types would get exclude designation here
    - Propose not supporting exclude_cash_only_tolls, include_hov2, include_hov3, hazmat, include_hot, hgv_no_access_penlty,  low_class_penalty, use_truck_route during early releases
## Road Types   
- Valhalla uses a "use" parameter to further apply travel restrictions in a request. The logical mapping from Overture for these parameter types would be from a combination of "class" and "subtype" attributes . With the proposed mapping below, passing in the specified "use" parameter would restrict routing to roads with the matched classification.   
- Use   
    - tram   
    - road   
    - ramp   
    - turn\_channel   
    - track   
        - Overture Class = track
    - driveway   
        - Overture Class = driveway
    - alley   
        - Overture Class = alley   
    - parking\_aisle   
        - Overture Class = parking\_aisle
    - emergency\_access
    - drive\_through
    - culdesac
    - cycleway   
       - Overture Class = cycleway
    - mountain\_bike
    - sidewalk   
       - Overture Class = sidewalk
    - footway   
       - Overture Class = footway   
       - Overture Class = path   
       - Overture Class = living\_street
    - steps   
       - Overture Class = steps
    - other
    - rail-ferry
    - ferry
    - rail   
      - Overture Subtype = rail
    - bus
    - egress\_connection
    - platform\_connection
    - transit\_connection   
