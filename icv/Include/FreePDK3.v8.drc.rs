// 3nm FreePDK(TM) V8 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV8_1 @= { @ "V8.1 : V8 shape is square";
	//Sushant
    not_aspect_ratio( aV8, direction = SHORT_BY_LONG, orientation = ORTHOGONAL, ratio = == 1 );
}

rV8_2 @= { @ "V8.2 : V8 is a square with 40nm edge length";
	//Sushant
    aspect_ratio( rectangles( aV8, sides = { != 0.04, != 0.04 } ), direction = SHORT_BY_LONG, ratio = == 1 );    
}

rV8_3 @= { @ "V8.3 : Minimum spacing of V8 - Full alignment >= 40nm";
	//Sushant
    external1(aV8, < 0.04, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external1(aV8, < 0.04, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);  
}

rV8_4 @= { @ "V8.4 : V8 should be enclosed between M8 and M9";
	//Sushant
    not_interacting( aV8, aM8 );
    not_interacting( aV8, aM9 );
}

rV8_5 @= { @ "V8.5 : V8 enclosure by M8 on two opposite sides, horizontal direction >= 10nm";
	//Sushant
    sV8_VERTICAL_EDGES = angle_edge( aV8, == 90 );
    sM8_VERTICAL_EDGES = angle_edge( aM8, == 90 );
    enclose_error( sV8_VERTICAL_EDGES, sM8_VERTICAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV8 not aM8;
}

rV8_6 @= { @ "V8.6 : V8 enclosure by M8 on two opposite sides, vertical direction = 0nm";
	//Sushant
    // sM8_HORIZONTAL_EDGES = angle_edge( aM8, == 0 );
	// and_edge( sM8_HORIZONTAL_EDGES, aV8, false ); 
	aV8 not aM8;
}

rV8_7 @= { @ "V8.7 : V8 enclosure by M9 on two opposite sides, horizontal direction = 0nm";
	//Sushant
    // sM9_VERTICAL_EDGES = angle_edge( aM9, == 90 );
	// and_edge( sM9_VERTICAL_EDGES, aV8, false );
	aV8 not aM9;
}

rV8_8 @= { @ "V8.8 : V8 enclosure by M9 on two opposite sides, vertical direction >= 10nm";
	//Sushant
    sV8_HORIZONTAL_EDGES = angle_edge( aV8, == 0 );
    sM9_HORIZONTAL_EDGES = angle_edge( aM9, == 0 );
    enclose_error( sV8_HORIZONTAL_EDGES, sM9_HORIZONTAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV8 not aM9;
}

rV8_9 @= { @ "V8.9 : Minimum area overlap between M8 and V8 >= 1600nm^2";
	//Sushant
    gV9 = and(aM8, aV8);
	area(gV9, < 0.001600);
}
