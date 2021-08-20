// 3nm FreePDK(TM) M0A ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM0A_1 @= { @ "M0A.1 : Minimum horizontal width of M0A >= 15 nm";
    //Sushant
	internal1( aM0A, < 0.015, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
}

rM0A_2 @= { @ "M0A.2 : Minimum spacing of M0A to GATE >= 6 nm";
    //Sushant
	external2(aM0A, aGATE, < 0.006, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM0A_3 @= { @ "M0A.3 : Minimum extension of ACT past M0A (horizontal direction) >= 5 nm";
    //Sushant
	sACT_VERTICAL_EDGES = angle_edge( aACT, == 90 );
    sM0A_VERTICAL_EDGES = angle_edge( aM0A, == 90 );
    enclose_error( sM0A_VERTICAL_EDGES, sACT_VERTICAL_EDGES, < 0.005, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );

}

rM0A_4 @= { @ "M0A.4 : Minimun vertical length of M0A >= 21.5 nm";
    //Sushant
	internal1( aM0A, < 0.0215, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rM0A_5 @= { @ "M0A.5 : Minimum vertical spacing of M0A >= 10 nm";
    //Sushant
	external1(aM0A, < 0.010, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM0A_6 @= { @ "M0A.6 : M0A may not bend";
    //Sushant
	not_rectangles(aM0A);
}

rM0A_7 @= { @ "M0A.7 : Minimum vertical overlap between M0A and ACTIVE  >= 11 nm";
    //Sushant
	gGRM0A7_check = and(aM0A, aACT);
	internal1 (gGRM0A7_check, < 0.011, direction = VERTICAL, extension = NONE);

}

rM0A_8 @= { @ "M0A.8 : M0A may not exitst without ACTIVE ";
    //Sushant
	//gGRM0A8_check = and(aM0A, aACT);
	//area(gGRM0A8_check, < 0.000165);
	//not_inside(aM0A, aACT);
	//aM0A_p = df_polygon_layer(aM0A);
	//aM0A_ACT_p = df_polygon_layer(gGRM0A8_check);
	//gGRM0A8_check1 = df_polygon_count(aM0A_p);
	//gGRM0A8_check2 = df_polygon_count(aM0A_ACT_p);
	//gGRM0A8_check1 != gGRM0A8_check2;
	not_cutting(aM0A, aACT, count > 0);
}

rM0A_9 @= { @ "M0A.9 : Minimum M0A area >= 322.5 nm^2";
    //Sushant
	area( aM0A, < 0.0003225);
}
