// 3nm FreePDK(TM) V2 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV2_1 @= { @ "V2.1 : V2 shape is square";
// Sushant 
    not_aspect_ratio( aV2, direction = SHORT_BY_LONG, orientation = ORTHOGONAL, ratio = == 1 );
}

rV2_2 @= { @ "V2.2 : V2 is a square with 14nm edge length";
// Sushant 
    aspect_ratio( rectangles( aV2, sides = { != 0.014, != 0.014 } ), direction = SHORT_BY_LONG, ratio = == 1 );    
}

rV2_3 @= { @ "V2.3 : Minimum spacing of V2 - Full alignment >= 15nm";
   //Sushant
	external1(aV2, < 0.015, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external1(aV2, < 0.015, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);   
}

rV2_4 @= { @ "V2.4 : V2 should be enclosed between M2 and M3";
    //Sushant
    not_interacting( aV2, aM2 );
    not_interacting( aV2, aM3 );
}

rV2_5 @= { @ "V2.5 : V2 enclosure by M2 on two opposite sides, horizontal direction >= 3nm";
	//Sushant
    sV2_VERTICAL_EDGES = angle_edge( aV2, == 90 );
    sM2_VERTICAL_EDGES = angle_edge( aM2, == 90 );
    enclose_error( sV2_VERTICAL_EDGES, sM2_VERTICAL_EDGES, < 0.003, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV2 not aM2;
}

rV2_6 @= { @ "V2.6 : V2 enclosure by M2 on two opposite sides, vertical direction = 0nm";
	//Sushant
    // sM2_HORIZONTAL_EDGES = angle_edge( aM2, == 0 );
	// and_edge( sM2_HORIZONTAL_EDGES, aV2, false );
	aV2 not aM2;
}

rV2_7 @= { @ "V2.7 : V2 enclosure by M3 on two opposite sides, horizontal direction = 0nm";
	//Sushant
    // sM3_VERTICAL_EDGES = angle_edge( aM3, == 90 );
	// and_edge( sM3_VERTICAL_EDGES, aV2, false );
	aV2 not aM3;
}

rV2_8 @= { @ "V2.8 : V2 enclosure by M3 on two opposite sides, vertical direction >= 3nm";
	//Sushant
    sV2_HORIZONTAL_EDGES = angle_edge( aV2, == 0 );
    sM3_HORIZONTAL_EDGES = angle_edge( aM3, == 0 );
    enclose_error( sV2_HORIZONTAL_EDGES, sM3_HORIZONTAL_EDGES, < 0.003, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV2 not aM3;
}

rV2_9 @= { @ "V2.9 : Minimum area overlap between M2 and V2 >= 196nm^2";
	//Sushant
    gV10 = and(aM2, aV2);
	area(gV10, < 0.000196);
}
