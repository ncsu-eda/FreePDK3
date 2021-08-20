// 3nm FreePDK(TM) V1 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV1_1 @= { @ "V1.1 : V1 shape is a square";
// Sushant 
    not_aspect_ratio( aV1, direction = SHORT_BY_LONG, orientation = ORTHOGONAL, ratio = == 1 );
}

rV1_2 @= { @ "V1.2 : V1 is a square with 14nm edge length";
// Sushant 
    aspect_ratio( rectangles( aV1, sides = { != 0.014, != 0.014 } ), direction = SHORT_BY_LONG, ratio = == 1 );    
}

rV1_3 @= { @ "V1.3 : Minimum spacing of V1 - Full alignment >= 14 nm";
   //Sushant
	external1(aV1, < 0.014, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external1(aV1, < 0.014, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);   
}


rV1_4 @= { @ "V1.4 : V1 should be enclosed between M1 and M2";
    //Sushant
    not_interacting( aV1, aM1 );
    not_interacting( aV1, aM2 );
}

rV1_5 @= { @ "V1.5 : V1 enclosure by M1 on two opposite sides, horizontal direction = 0nm";
	//Sushant
    //sM1_VERTICAL_EDGES = angle_edge( aM1, == 90 );
	//and_edge( sM1_VERTICAL_EDGES, aV1, false );
	aV1 not aM1;
}

rV1_6 @= { @ "V1.6 : V1 enclosure by M1 on two opposite sides, vertical direction >= 3nm";
	//Sushant
    sV1_HORIZONTAL_EDGES = angle_edge( aV1, == 0 );
    sM1_HORIZONTAL_EDGES = angle_edge( aM1, == 0 );
    enclose_error( sV1_HORIZONTAL_EDGES, sM1_HORIZONTAL_EDGES, < 0.003, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV1 not aM1;
}

rV1_7 @= { @ "V1.7 : V1 enclosure by M2 on two opposite sides, horizontal direction >= 3nm";
	//Sushant
    sV1_VERTICAL_EDGES = angle_edge( aV1, == 90 );
    sM2_VERTICAL_EDGES = angle_edge( aM2, == 90 );
    enclose_error( sV1_VERTICAL_EDGES, sM2_VERTICAL_EDGES, < 0.003, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV1 not aM2;
}

rV1_8 @= { @ "V1.8 : V1 enclosure by M2 on two opposite sides, vertical direction = 0nm";
	//Sushant
    // sM2_HORIZONTAL_EDGES = angle_edge( aM2, == 90 );
	// and_edge( sM2_HORIZONTAL_EDGES, aV1, false );
	aV1 not aM2;
}

rV1_9 @= { @ "V1.9 : Minimum area overlap between M1 and V1 = 19nm^2";
	//Sushant
    gV10 = and(aM1, aV1);
	area(gV10, < 0.000196);
}
