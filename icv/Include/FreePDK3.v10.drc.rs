// 3nm FreePDK(TM) V10 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV10_1 @= { @ "V10.1 : V10 shape is square";
	//Sushant
    not_aspect_ratio( aV10, direction = SHORT_BY_LONG, orientation = ORTHOGONAL, ratio = == 1 );
}

rV10_2 @= { @ "V10.2 : V10 is a square with 80nm edge length";
	//Sushant
    aspect_ratio( rectangles( aV10, sides = { != 0.08, != 0.08 } ), direction = SHORT_BY_LONG, ratio = == 1 );  
}

rV10_3 @= { @ "V10.3 : Minimum spacing of V10 - Full alignment >= 80nm";
	//Sushant
    external1(aV10, < 0.08, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external1(aV10, < 0.08, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);
}

rV10_4 @= { @ "V10.4 : V10 should be enclosed between M10 and M11";
	//Sushant
    not_interacting( aV10, aM10 );
    not_interacting( aV10, aM11 );
}

rV10_5 @= { @ "V10.5 : V10 enclosure by M10 on two opposite sides, horizontal direction >= 10nm";
	//Sushant
    sV10_VERTICAL_EDGES = angle_edge( aV10, == 90 );
    sM10_VERTICAL_EDGES = angle_edge( aM10, == 90 );
    enclose_error( sV10_VERTICAL_EDGES, sM10_VERTICAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV10 not aM10;
}

rV10_6 @= { @ "V10.6 : V10 enclosure by M10 on two opposite sides, vertical direction = 0nm";
	//Sushant
    // sM10_HORIZONTAL_EDGES = angle_edge( aM10, == 0 );
	// and_edge( sM10_HORIZONTAL_EDGES, aV10, false ); 
	aV10 not aM10;
}

rV10_7 @= { @ "V10.7 : V10 enclosure by M11 on two opposite sides, horizontal direction = 0nm";
	//Sushant
    // sM11_VERTICAL_EDGES = angle_edge( aM11, == 90 );
	// and_edge( sM11_VERTICAL_EDGES, aV10, false );
	aV10 not aM11;
}

rV10_8 @= { @ "V10.8 : V10 enclosure by M11 on two opposite sides, vertical direction >= 10nm";
	//Sushant
    sV10_HORIZONTAL_EDGES = angle_edge( aV10, == 0 );
    sM11_HORIZONTAL_EDGES = angle_edge( aM11, == 0 );
    enclose_error( sV10_HORIZONTAL_EDGES, sM11_HORIZONTAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV10 not aM11; 
}

rV10_9 @= { @ "V10.9 : Minimum area overlap between M10 and V10 >= 6400nm^2";
	//Sushant
    gV9 = and(aM10, aV10);
	area(gV9, < 0.006400);
}