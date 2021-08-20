// 3nm FreePDK(TM) GCON ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rGCON_1 @= { @ "GCON.1 : Exact VERTICAL width of GCON = 13 nm";
    //Sushant
	gGRGCON1_check = internal1( aGCON, == 0.013, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aGCON not gGRGCON1_check;
}

rGCON_2 @= { @ "GCON.2 : Exact HORIZONTAL length of GCON = 15 nm";
    //Sushant
	gGRGCON2_check = internal1( aGCON, == 0.015, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aGCON not gGRGCON2_check;
}

rGCON_3 @= { @ "GCON.3 : GCON must overlap gate";
    not_interacting( aGCON, aGATE, include_touch = NONE );
}

rGCON_4 @= { @ "GCON.4 : GCON extention past M0B in vertical direction >= 1 nm";
    //Sushant
//	gGRGCON4_check = enclose( aGCON, aM0B, distance < 1, direction = HORIZONTAL );
//    aGCON not gGRGCON4_check;
	enclose( aM0B, aGCON, < 0.001, extension = NONE, direction = VERTICAL);
}

rGCON_5 @= { @ "GCON.5 : Minimum horizontal spacing between two GCON layer >= 25 nm";
    //Sushant
	external1(aGCON, < 0.025, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);
}

rGCON_6 @= { @ "GCON.6 : Minimum vertical spacing between two GCON layer >= 28 nm";
    //Sushant
	external1(aGCON, < 0.028, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);  
}

rGCON_7 @= { @ "GCON.7 : Minimum spacing between GCON and M0A layer polygons >= 6 nm";
    //Sushant
	external2(aGCON, aM0A, < 0.006, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 	
}

rGCON_8 @= { @ "GCON.8 : Minimum horizontal spacing between GCON and GATE layer >= 25 nm";
    //Sushant
	external2( aGCON, aGATE, < 0.025, connectivity = DIFFERENT_NET, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT, connect_sequence = CONNECT_DB);
}

rGCON_9 @= { @ "GCON.9 : Minimum vertical spacing between GCON and GCUT >= 13.5 nm";
    //Sushant
	external2(aGCON, aGCUT, < 0.0135, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);
}

rGCON_10 @= { @ "GCON.10 : Minimum GCON Area >= 180 nm^2";
    //Sushant
	area( aGCON, < 0.000180);
}

rGCON_11 @= { @ "GCON.11 : GCON may not bend";
    //Sushant
	not_rectangles(aGCON);
}

rGCON_12 @= { @ "GCON.12 : Minumum area of overlap between GCON and M0B >= 165 nm^2";
    gGCON12 = interacting( aGCON, aM0B );
    area( gGCON12, < 0.000165 );
}

rGCON_13 @= { @ "GCON.13 : Minimum area of overlap between GCON and GATE  >= 195 nm^2";
    //Sushant
	gGCON13 = interacting( aGCON, aGATE );
    area( gGCON13, < 0.000195 );
}

rGCON_14 @= { @ "GCON.14 : GCON may not  interact with GCUT or DUMMY";
    //Sushant
	gGRGCON14_check = or(aGCUT, aDUMMY);
    interacting( aGCON, gGRGCON14_check, include_touch = ALL);
}

