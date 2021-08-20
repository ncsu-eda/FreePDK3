// 3nm FreePDK(TM) GCUT ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rGCUT_1 @= { @ "GCUT.1 : Exact vertical width of GCUT(shape is oriented horizontally) = 10.5 nm";
    //Sushant
	gGRGCUT1_check = internal1( aGCUT, == 0.0105, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aGCUT not gGRGCUT1_check;
}

rGCUT_2 @= { @ "GCUT.2 : Minimum horizontal length of GCUT(shape is oriented horizontally) >= 42 nm";
    //Sushant
	internal1( aGCUT, < 0.042, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );

}

rGCUT_3 @= { @ "GCUT.3 : Minimum horizontal space of GCUT >= 69 nm";
    //Sushant
	external1(aGCUT, < 0.069, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rGCUT_4 @= { @ "GCUT.4 : GCUT minimum space to ACT >= 20.5 nm";
    //Sushant
	external2(aGCUT, aACT, < 0.0205, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);
}

rGCUT_5 @= { @ "GCUT.5 : GCUT may not bend";
    //Sushant
	not_rectangles(aGCUT);
}

rGCUT_6 @= { @ "GCUT.6 : GCUT vertical edge must coinside with DUMMY vertical edge";
    gGCUT_v = angle_edge( aGCUT, == 90);
    and_edge( gGCUT_v, aDUMMY, false);
}

rGCUT_7 @= { @ "GCUT.7 : GCUT layer may not exist without the layer GATE";
    //Sushant
	not_cutting(aGCUT, aGATE, count > 0);
}

rGCUT_8 @= { @ "GCUT.8 : GCUT layer vertical edge may not lie inside, or coincide with, the GATE layer ";
    //Sushant
	//gGCUT_v = angle_edge( aGCUT, == 90);
    //and_edge( gGCUT_v, aGATE);
}

rGCUT_9 @= { @ "GCUT.9 : GCUT may not interact with ACT";
    //Sushant
	//gGRGCUT9_check = or(aACT);
	interacting( aGCUT, aACT, include_touch = ALL);
}

rGCUT_10 @= { @ "GCUT.10 : Minimum vertical spacing between two GCUT layer >= 105 nm";
    //Sushant
	external1(aGCUT, < 0.105, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);   
}



