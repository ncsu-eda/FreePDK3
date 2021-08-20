// 3nm FreePDK(TM) V7 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV7_1 @= { @ "V7.1 : V7 shape is square";
	//Sushant
    not_aspect_ratio( aV7, direction = SHORT_BY_LONG, orientation = ORTHOGONAL, ratio = == 1 );
}

rV7_2 @= { @ "V7.2 : V7 is a square with 40nm edge length";
	//Sushant
    aspect_ratio( rectangles( aV7, sides = { != 0.04, != 0.04 } ), direction = SHORT_BY_LONG, ratio = == 1 );    
}

rV7_3 @= { @ "V7.3 : Minimum spacing of V7 - Full alignment >= 40nm";
	//Sushant
    external1(aV7, < 0.04, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external1(aV7, < 0.04, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);   
}

rV7_4 @= { @ "V7.4 : V7 should be enclosed between M7 and M8";
	//Sushant
    not_interacting( aV7, aM7 );
    not_interacting( aV7, aM8 );
}

rV7_5 @= { @ "V7.5 : V7 enclosure by M7 on two opposite sides, horizontal direction = 0nm";
	//Sushant
    // sM7_VERTICAL_EDGES = angle_edge( aM7, == 90 );
	// and_edge( sM7_VERTICAL_EDGES, aV7, false );
	aV7 not aM7;	
}

rV7_6 @= { @ "V7.6 : V7 enclosure by M7 on two opposite sides, vertical direction >= 10nm";
	//Sushant
    sV7_HORIZONTAL_EDGES = angle_edge( aV7, == 0 );
    sM7_HORIZONTAL_EDGES = angle_edge( aM7, == 0 );
    enclose_error( sV7_HORIZONTAL_EDGES, sM7_HORIZONTAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV7 not aM7; 
}

rV7_7 @= { @ "V7.7 : V7 enclosure by M8 on two opposite sides, horizontal direction >= 10nm";
	//Sushant
    sV7_VERTICAL_EDGES = angle_edge( aV7, == 90 );
    sM8_VERTICAL_EDGES = angle_edge( aM8, == 90 );
    enclose_error( sV7_VERTICAL_EDGES, sM8_VERTICAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV7 not aM8; 
}

rV7_8 @= { @ "V7.8 : V7 enclosure by M8 on two opposite sides, vertical direction = 0nm";
	//Sushant
    // sM8_HORIZONTAL_EDGES = angle_edge( aM8, == 90 );
	// and_edge( sM8_HORIZONTAL_EDGES, aV7, false );
	aV7 not aM8; 
}

rV7_9 @= { @ "V7.9 : Minimum area overlap between M7 and V7 >= 1600 nm^2";
	//Sushant
    gV9 = and(aM7, aV7);
	area(gV9, < 0.001600);
}