// 3nm FreePDK(TM) V3 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV3_1 @= { @ "V3.1 : V3 shape is rectangle";
	//Sushant
    not_rectangles(aV3); 
}

rV3_2 @= { @ "V3.2 : V3 is a rectangle with 15nm horizontal edge and 24nm vertical edge";
	//Sushant
    gGRV3_1_check = internal1( aV3, == 0.015, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aV3 not gGRV3_1_check;
    gGRV3_2_check = internal1( aV3, == 0.024, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aV3 not gGRV3_2_check;	 
}

rV3_3a @= { @ "V3.3a : Minimum horizontal spacing of V3 - Full alignment >= 15nm";
	//Sushant
    external1(aV3, < 0.015, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rV3_3b @= { @ "V3.3b : Minimum vertical spacing of V3 - Full alignment >= 24nm";
	//Sushant
    external1(aV3, < 0.024, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rV3_4 @= { @ "V3.4 : V3 should be enclosed between M3 and M4";
	//Sushant
    not_interacting( aV3, aM3 );
    not_interacting( aV3, aM4 );
}

rV3_5 @= { @ "V3.5 : V3 enclosure by M3 on two opposite sides, horizontal direction == 0nm";
	//Sushant
    // sM3_VERTICAL_EDGES = angle_edge( aM3, == 90 );
	// and_edge( sM3_VERTICAL_EDGES, aV3, false );
	aV3 not aM3;
}

rV3_6 @= { @ "V3.6 : V3 enclosure by M3 on two opposite sides, vertical direction >= 3nm";
	//Sushant
    sV3_HORIZONTAL_EDGES = angle_edge( aV3, == 0 );
    sM3_HORIZONTAL_EDGES = angle_edge( aM3, == 0 );
    enclose_error( sV3_HORIZONTAL_EDGES, sM3_HORIZONTAL_EDGES, < 0.003, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV3 not aM3;
}

rV3_7 @= { @ "V3.7 : V3 enclosure by M4 on two opposite sides, horizontal direction >= 3nm";
	//Sushant
    sV3_VERTICAL_EDGES = angle_edge( aV3, == 90 );
    sM4_VERTICAL_EDGES = angle_edge( aM4, == 90 );
    enclose_error( sV3_VERTICAL_EDGES, sM4_VERTICAL_EDGES, < 0.003, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV3 not aM4;
}

rV3_8 @= { @ "V3.8 : V3 enclosure by M4 on two opposite sides, vertical direction == 0nm";
	//Sushant
    // sM4_HORIZONTAL_EDGES = angle_edge( aM4, == 90 );
	// and_edge( sM4_HORIZONTAL_EDGES, aV3, false );
	aV3 not aM4;
}

rV3_9 @= { @ "V3.9 : Minimum area overlap between M3 and V3 >= 360nm^2";
	//Sushant
    gV11 = and(aM3, aV3);
	area(gV11, < 0.000360); 
}