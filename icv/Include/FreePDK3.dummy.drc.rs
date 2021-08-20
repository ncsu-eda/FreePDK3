// 3nm FreePDK(TM) DUMMY ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

// Sushant
rDUMMY_1 @= { @ "DUMMY.1 : DUMMY exact horizontal width = 15 nm";
    // Sushant
	gGRDUMMY1_check = internal1( aDUMMY, == 0.015, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aDUMMY not gGRDUMMY1_check;
}

rDUMMY_2 @= { @ "DUMMY.2 : DUMMY minimum vertical length >= 40 nm";
    // Sushant
	internal1( aDUMMY, < 0.040, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rDUMMY_3 @= { @ "DUMMY.3 : Minimum vertical space >= 115.5 nm";
    // Sushant
	//TBD
	external1(aDUMMY, < 0.1155, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rDUMMY_4 @= { @ "DUMMY.4 : DUMMY may not bend";
    // Sushant
	not_rectangles(aDUMMY);
}

rDUMMY_5 @= { @ "DUMMY.5: DUMMY must completely overlap GATE"; 
	aDUMMY not_inside aGATE;
}

rDUMMY_6 @= { @ "DUMMY.6 : Dummy horizontal edge maynot lie inside ACTIVE"; 
	gDUMMY_h = angle_edge( aDUMMY, ==0);
	and_edge(gDUMMY_h, aACT);
}
