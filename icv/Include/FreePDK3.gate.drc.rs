// 3nm FreePDK(TM) GATE ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

// Sushant

rGATE_1 @= { @ "GATE.1 : GATE exact horizontal width = 15 nm";
    // Sushant
	gGRGATE1_check = internal1( aGATE, == 0.015, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aGATE not gGRGATE1_check;
}

rGATE_2 @= { @ "GATE.2 : Minimum horizontal spacing between GATE or DUMMY layers >= 27 nm";
    // Sushant
	external1(aGATE, < 0.027, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external2(aDUMMY, aGATE, < 0.027, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rGATE_3 @= { @ "GATE.3 : GATE may not bend";
    // Sushant
	not_rectangles(aGATE);
}

rGATE_4 @= { @ "GATE.4 : GATE min extension past ACT in vertical direction >= 21.5 nm";
    // TBD
	enclose( aACT, aGATE, < 0.0215, extension = NONE, direction = VERTICAL);
}


rGATE_5 @= { @ "GATE.5 : GATE minimum vertical length >= 40 nm";
    // Sushant
	internal1( aGATE, < 0.040, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rGATE_6 @= { @ "GATE.6 : GATE may not be discontinuous along the vertical axis. Use GCUT layer to mark cuts in the GATE ";
    // Sushant
	external1(aGATE, < 1, direction = VERTICAL, extension = NONE);	
}

rGATE_7 @= { @ "GATE.7 : ACT layer vertical edge may not lie inside, or coincide with, the GATE layer ";
    // Sushant
	//gACT_v = angle_edge( aACT, == 90);
    //and_edge( gACT_v, aGATE);

}

rGATE_8 @= { @ "GATE.8 : Minimum horizontal spacing between ACT and GATE (not cut by GCUT and not interacting with ACT) >= 6 nm";
    // Sushant
	external2( aGATE, aACT, < 0.006, connectivity = DIFFERENT_NET, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT, connect_sequence = CONNECT_DB);
}



