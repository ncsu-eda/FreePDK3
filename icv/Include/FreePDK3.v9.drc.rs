// 3nm FreePDK(TM) V9 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV9_1 @= { @ "V9.1 : V9 shape is rectangle";
	//Sushant
    not_rectangles(aV9);
}

rV9_2 @= { @ "V9.2 : V9 is a rectangle with 40nm horizontal edge and 80nm vertical edge";
	//Sushant
    gGRV9_1_check = internal1( aV9, == 0.04, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aV9 not gGRV9_1_check;
    gGRV9_2_check = internal1( aV9, == 0.08, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aV9 not gGRV9_2_check;
}

rV9_3a @= { @ "V9.3a : Minimum horizontal spacing of V9 - Full alignment >= 40nm";
	//Sushant
    external1(aV9, < 0.04, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rV9_3b @= { @ "V9.3b : Minimum vertical spacing of V9 - Full alignment >= 80nm";
	//Sushant
    external1(aV9, < 0.08, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rV9_4 @= { @ "V9.4 : V9 should be enclosed between M9 and M10";
	//Sushant
    not_interacting( aV9, aM9 );
    not_interacting( aV9, aM10 );
}

rV9_5 @= { @ "V9.5 : V9 enclosure by M9 on two opposite sides, horizontal direction = 0nm";
	//Sushant
    // sM9_VERTICAL_EDGES = angle_edge( aM9, == 90 );
	// and_edge( sM9_VERTICAL_EDGES, aV9, false ); 
	aV9 not aM9;
}

rV9_6 @= { @ "V9.6 : V9 enclosure by M9 on two opposite sides, vertical direction >= 10nm";
	//Sushant
    sV9_HORIZONTAL_EDGES = angle_edge( aV9, == 0 );
    sM9_HORIZONTAL_EDGES = angle_edge( aM9, == 0 );
    enclose_error( sV9_HORIZONTAL_EDGES, sM9_HORIZONTAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV9 not aM9;
}

rV9_7 @= { @ "V9.7 : V9 enclosure by M10 on two opposite sides, horizontal direction >= 10nm";
	//Sushant
    sV9_VERTICAL_EDGES = angle_edge( aV9, == 90 );
    sM10_VERTICAL_EDGES = angle_edge( aM10, == 90 );
    enclose_error( sV9_VERTICAL_EDGES, sM10_VERTICAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV9 not aM10;
}

rV9_8 @= { @ "V9.8 : V9 enclosure by M10 on two opposite sides, vertical direction = 0nm";
	//Sushant
    // sM10_HORIZONTAL_EDGES = angle_edge( aM10, == 90 );
	// and_edge( sM10_HORIZONTAL_EDGES, aV9, false );
	aV9 not aM10;
}

rV9_9 @= { @ "V9.9 : Minimum area overlap between M9 and V9 >= 3200nm^2";
	//Sushant
    gV10 = and(aM9, aV9);
	area(gV10, < 0.003200);
}