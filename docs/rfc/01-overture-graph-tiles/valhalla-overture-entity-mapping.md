# Bifrost Entity Mapping   
- Author: Rob Haley (rhaley@esri.com)
- Status: Draft
- Created: 2025-06-09
- Last Updated: 2025-06-19
1. Road Classification   
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
        | parking\_asile |     kService\_Other |
        |           path |     kService\_Other |
        |       sidewalk |     kService\_Other |
        |          steps |     kService\_Other |
        |          track |     kService\_Other |
        |        unknown |       kUnclassified |

2. Access Restrictions   
    - Access restrictions in Overture currently have a lot of needless complexity. Roads can get one of three assignment types: designated, allowed or denied. Those assignments then get a mode of travel assigned to them. A better long term approach would be for Overture to utilize only the denied assignment and assume all modes of transit are available for any road absent that assignment. That would greatly simplify the structure and reduce all the opportunities for conflict.   
    - Because of the opportunities for conflict, a remaining effort for this document is to resolve conflicts by creating a tiering system.   
    - Currently within Overture there is an innumerable amount combinations of assignments that could come in for a specific extent. A cost of this is that in order to know all the assignments that would need to be supported for a given extent, all the roads must be read in. A simpler approach and a future design clarification should be setting a fixed amount of assignment that should be supported, assigning default values when those are not present and ignoring unsupported values.   
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
3. Future   
    1. kBikeshareAccess   
    2. kTaxiAccess   
    3. kMotor\_scooterAccess   
    4. kMotorcycleAccess   
4. Valhalla makes use of bidirectional directed
edge restrictions by separating them into start\_restriction and
end\_restriction. The general structure end up looking like this:

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
    - Overture designates travel rules
with either a backwards or forwards designation in the when: heading: like so:   
   
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
5. Speed Limits   
    - Work in progress   
6. Surface Types   
    - Work in progress   
7. Direction of Travel   
    - Work in progress   
8. Vehicle Options   
    - Work in progress   
    - Height   
    - Width   
    - Length   
    - Weight   
9. Special    
    1. Work in progress   
10. Road Types   
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
