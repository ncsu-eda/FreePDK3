// 3nm FreePDK(TM) V4 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV4_1 @= { @ "V4.1 : V4 shape is square";
	//Sushant
    not_aspect_ratio( aV4, direction = SHORT_BY_LONG, orientation = ORTHOGONAL, ratio = == 1 );
}

rV4_2 @= { @ "V4.2 : V4 is a square with 24nm edge length";
	//Sushant
    aspect_ratio( rectangles( aV4, sides = { != 0.024, != 0.024 } ), direction = SHORT_BY_LONG, ratio = == 1 );  
}

rV4_3 @= { @ "V4.3 : Minimum spacing of V4 - Full alignment >= 24nm";
	//Sushant
    external1(aV4, < 0.024, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);  
	
	external1(aV4, < 0.024, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rV4_3_1 @= { @ "V4.3 : Minimum spacing of V4 - Full alignment >= 24nm";
	//Sushant
	  
}

rV4_4 @= { @ "V4.4 : V4 should be enclosed between M4 and M5";
	//Sushant
    not_interacting( aV4, aM4 );
    not_interacting( aV4, aM5 );
}

rV4_5 @= { @ "V4.5 : V4 enclosure by M4 on two opposite sides, horizontal direction >= 10nm";
	//Sushant
    sV4_VERTICAL_EDGES = angle_edge( aV4, == 90 );
    sM4_VERTICAL_EDGES = angle_edge( aM4, == 90 );
    enclose_error( sV4_VERTICAL_EDGES, sM4_VERTICAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV4 not aM4;
}

rV4_6 @= { @ "V4.6 : V4 enclosure by M4 on two opposite sides, vertical direction == 0nm";
	//Sushant
    // sM4_HORIZONTAL_EDGES = angle_edge( aM4, == 0 );
	// and_edge( sM4_HORIZONTAL_EDGES, aV4, false ); 
	aV4 not aM4;
}

rV4_7 @= { @ "V4.7 : V4 enclosure by M5 on two opposite sides, horizontal direction == 0nm";
	//Sushant
    // sM5_VERTICAL_EDGES = angle_edge( aM5, == 90 );
	// and_edge( sM5_VERTICAL_EDGES, aV4, false );
	aV4 not aM5;
}

rV4_8 @= { @ "V4.8 : V4 enclosure by M5 on two opposite sides, vertical direction >= 10nm";
	//Sushant
    sV4_HORIZONTAL_EDGES = angle_edge( aV4, == 0 );
    sM5_HORIZONTAL_EDGES = angle_edge( aM5, == 0 );
    enclose_error( sV4_HORIZONTAL_EDGES, sM5_HORIZONTAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV4 not aM5; 
}

rV4_9 @= { @ "V4.9 : Minimum area overlap between M4 and V4 >= 576nm^2";
	//Sushant
    gV9 = and(aM4, aV4);
	area(gV9, < 0.000576);     
}