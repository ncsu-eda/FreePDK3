// 3nm FreePDK(TM) V0A ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV0A_1 @= { @ "V0A.1 : V0A shape is a square";
    not_aspect_ratio( aV0A, direction = SHORT_BY_LONG, orientation = ORTHOGONAL, ratio = == 1 );
}

rV0A_2 @= { @ "V0A.2 : V0A is a square with 13 nm edge length";
    aspect_ratio( rectangles( aV0A, sides = { != 0.013, != 0.013 } ), direction = SHORT_BY_LONG, ratio = == 1 );
}

rV0A_3 @= { @ "V0A.3 : Minimum spacing of V0A - Full alignment >= 29 nm";
   //Sushant
	external1(aV0A, < 0.029, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external1(aV0A, < 0.029, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);   
}

rV0A_4 @= { @ "V0A.4 : Minimum corner-to-corner spacing between two V0A instances >= 30 nm";
   //Sushant
	external_corner1(aV0A, <0.030);
}

rV0A_5 @= { @ "V0A.5 : V0A must always interact with M0A and M0B";
    not_interacting( aV0A, aM0A );
    not_interacting( aV0A, aM0B );
}

rV0A_6 @= { @ "V0A.6 : V0A enclosure by M0B on two opposite sides, horizontal direction >= 6 nm";
   //Sushant
	sV0A_VERTICAL_EDGES = angle_edge( aV0A, == 90 );
    sM0B_VERTICAL_EDGES = angle_edge( aM0B, == 90 );
    enclose_error( sV0A_VERTICAL_EDGES, sM0B_VERTICAL_EDGES, < 0.006, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT, relational = OUTSIDE  );	
	and_edge(sM0B_VERTICAL_EDGES, aV0A);
}

rV0A_7 @= { @ "V0A.7 : V0A enclosure by M0B on two opposite sides, vertical direction = -1 nm";
   //Sushant
	gCHECK_1i = and(aV0A, aM0B);
    gCHECK_2i = grow(gCHECK_1i, north = 0.001, south = 0.001);
    not(aV0A, gCHECK_2i);
    not(gCHECK_2i, aV0A);	
}

rV0A_8 @= { @ "V0A.8 : V0A enclosure by M0A on two opposite sides, horizontal direction = 1 nm";
   //Sushant
//	gGRV0A8_check = enclose( aV0A, aM0A, distance == 0.001, direction = HORIZONTAL );
//    aV0A not gGRV0A8_check;
	sV0A_VERTICAL_EDGES = angle_edge( aV0A, == 90 );
    sM0A_VERTICAL_EDGES = angle_edge( aM0A, == 90 );
    enclose_error( sV0A_VERTICAL_EDGES, sM0A_VERTICAL_EDGES, < 0.001, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
	//aV0A not aM0A;
	and_edge(sM0A_VERTICAL_EDGES, aV0A);
}

rV0A_9 @= { @ "V0A.9 : V0A enclosure by M0A on two opposite sides, vertical direction";
   //Sushant
	sV0A_HORIZONTAL_EDGES = angle_edge( aV0A, == 0 );
    sM0A_HORIZONTAL_EDGES = angle_edge( aM0A, == 0 );
    enclose_error( sM0A_HORIZONTAL_EDGES, sV0A_HORIZONTAL_EDGES, < 0.001, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );

}

rV0A_10 @= { @ "V0A.10 : Minimum space of V0A and M0B of different net >= 8 nm";
   //Sushant
	external2( aV0A, aM0B, < 0.008, connectivity = DIFFERENT_NET, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT, connect_sequence = CONNECT_DB);
}

rV0A_11 @= { @ "V0A.11 : Minimum space of V0A and M0A of different net >= 16.5 nm";
   //Sushant
	external2( aV0A, aM0A, < 0.0165, connectivity = DIFFERENT_NET, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT, connect_sequence = CONNECT_DB);
}

rV0A_12 @= { @ "V0A.12 : V0A may not  interact with GCUT or DUMMY or GATE layer";
   //Sushant
    gGRV0A12_check = or(aDUMMY, aGATE);
    interacting( aV0A, gGRV0A12_check, include_touch = ALL);
    interacting( aV0A, aDUMMY, include_touch = ALL);
}

rV0A_13 @= { @ "V0A.13 : Minimum area overlap between M0A and V0A >= 156 nm^2";
	gV0A13 = and(aM0A, aV0A);
	area(gV0A13, < 0.000156);
}
