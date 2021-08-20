// 3nm FreePDK(TM) VRDL ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rVRDL_1 @= { @ "VRDL.1 : VRDL shape is square";
    //Sushant
	not_aspect_ratio( aVRDL, direction = SHORT_BY_LONG, orientation = ORTHOGONAL, ratio = == 1 );
}

rVRDL_2 @= { @ "VRDL.2 : VRDL is a square with 160nm edge length";
    //Sushant
	aspect_ratio( rectangles( aVRDL, sides = { != 0.16, != 0.16 } ), direction = SHORT_BY_LONG, ratio = == 1 );
}

rVRDL_3 @= { @ "VRDL.3 : Minimum spacing of V12 - Full alignment >= 160nm";
	//Sushant
    external1(aVRDL, < 0.16, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external1(aVRDL, < 0.16, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);
}

rVRDL_4 @= { @ "VRDL.4 : VRDL must be enclosed between M13 and RDL layers";
	//Sushant
    not_interacting( aVRDL, aM13 );
    not_interacting( aVRDL, aRDL );
}

rVRDL_5 @= { @ "VRDL.5 : VRDL enclosure by M13 in horizontal direction >= 0nm";
	//Sushant
    aVRDL not aM13;
}

rVRDL_6 @= { @ "VRDL.6 : VRDL enclosure by M13 in vertical direction >= 10nm";
	//Sushant
    sVRDL_HORIZONTAL_EDGES = angle_edge( aVRDL, == 0 );
    sM13_HORIZONTAL_EDGES = angle_edge( aM13, == 0 );
    enclose_error( sVRDL_HORIZONTAL_EDGES, sM13_HORIZONTAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aVRDL not aM13;
}

rVRDL_7 @= { @ "VRDL.7 : VRDL enclosure by RDL in horizontal direction >= 80nm";
	//Sushant
    sVRDL_VERTICAL_EDGES = angle_edge( aVRDL, == 90 );
    sRDL_VERTICAL_EDGES = angle_edge( aRDL, == 90 );
    enclose_error( sVRDL_VERTICAL_EDGES, sRDL_VERTICAL_EDGES, < 0.08, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aVRDL not aRDL;
}

rVRDL_8 @= { @ "VRDL.8 : VRDL enclosure by RDL in vertical direction >= 80nm";
	//Sushant
    sVRDL_HORIZONTAL_EDGES = angle_edge( aVRDL, == 0 );
    sRDL_HORIZONTAL_EDGES = angle_edge( aRDL, == 0 );
    enclose_error( sVRDL_HORIZONTAL_EDGES, sRDL_HORIZONTAL_EDGES, < 0.08, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aVRDL not aRDL;
}

rVRDL_9 @= { @ "VRDL.9 : Minimum area overlap between M13 and VRDL >= 25600nm^2";
	//Sushant
    gVRDL = and(aM13, aVRDL);
	area(gVRDL, < 0.025600);
}