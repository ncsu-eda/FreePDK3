// 3nm FreePDK(TM) V5 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV5_1 @= { @ "V5.1 : V5 shape is square";
	//Sushant
    not_aspect_ratio( aV5, direction = SHORT_BY_LONG, orientation = ORTHOGONAL, ratio = == 1 );
}

rV5_2 @= { @ "V5.2 : V5 is a square with 24nm edge length";
	//Sushant
    aspect_ratio( rectangles( aV5, sides = { != 0.024, != 0.024 } ), direction = SHORT_BY_LONG, ratio = == 1 );
}

rV5_3 @= { @ "V5.3 : Minimum spacing of V5 - Full alignment >= 24nm";
	//Sushant
    external1(aV5, < 0.024, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external1(aV5, < 0.024, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);   
}

rV5_4 @= { @ "V5.4 : V5 should be enclosed between M5 and M6";
	//Sushant
    not_interacting( aV5, aM5 );
    not_interacting( aV5, aM6 ); 
}

rV5_5 @= { @ "V5.5 : V5 enclosure by M5 on two opposite sides, horizontal direction = 0nm";
	//Sushant
    // sM5_VERTICAL_EDGES = angle_edge( aM5, == 90 );
	// and_edge( sM5_VERTICAL_EDGES, aV5, false );
	aV5 not aM5;
}

rV5_6 @= { @ "V5.6 : V5 enclosure by M5 on two opposite sides, vertical direction >= 10nm";
	//Sushant
    sV5_HORIZONTAL_EDGES = angle_edge( aV5, == 0 );
    sM5_HORIZONTAL_EDGES = angle_edge( aM5, == 0 );
    enclose_error( sV5_HORIZONTAL_EDGES, sM5_HORIZONTAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV5 not aM5;
}

rV5_7 @= { @ "V5.7 : V5 enclosure by M6 on two opposite sides, horizontal direction >= 10nm";
	//Sushant
    sV5_VERTICAL_EDGES = angle_edge( aV5, == 90 );
    sM6_VERTICAL_EDGES = angle_edge( aM6, == 90 );
    enclose_error( sV5_VERTICAL_EDGES, sM6_VERTICAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV5 not aM6; 
}

rV5_8 @= { @ "V5.8 : V5 enclosure by M6 on two opposite sides, vertical direction = 0nm";
	//Sushant
    // sM6_HORIZONTAL_EDGES = angle_edge( aM6, == 90 );
	// and_edge( sM6_HORIZONTAL_EDGES, aV5, false ); 
	aV5 not aM6; 
}

rV5_9 @= { @ "V5.9 : Minimum area overlap between M5 and V5 >= 576nm^2";
	//Sushant
    gV9 = and(aM5, aV5);
	area(gV9, < 0.000576); 
}

