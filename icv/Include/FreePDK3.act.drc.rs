// 3nm FreePDK(TM) ACT ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rACT_1 @= { @ "ACT.1 : Minimum vertical width of ACT >= 21 nm";
// Sushant
	internal1( aACT, < 0.021, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rACT_2 @= { @ "ACT.2 : Minimum vertical spacing of ACT >= 21.5 nm";
    external1(aACT, < 0.0215, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rACT_3 @= { @ "ACT.3 : Minimum horizontal width of ACT >= 84 nm";
// Sushant 
	internal1( aACT, < 0.084, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
}

rACT_4 @= { @ "ACT.4 : ACT should be continuous";
	gACT_1i = aACT and aDUMMY;
    // check for the distance between two ACT polygons inside DUMMY; flag as violation if smaller than width of DUMMY
    external1(gACT_1i, < 0.015, direction = HORIZONTAL, extension = NONE);
	external1(aACT, < 1, direction = HORIZONTAL, extension = NONE);
}

rACT_5 @= { @ "ACT.5 : Minimum vertical spacing between ACT and BPR >= 10 nm";
    external2(aACT, aBPR, < 0.010, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);   
}

rACT_6 @= { @ "ACT.6 : ACT may not bend";
    not_rectangles(aACT);
}

rACT_7 @= { @ "ACT.7 : ACT must end inside a DUMMY layer";
// TBD
	gACT_lineend = angle_edge(aACT, ==90); 
	gACT_lineend not_edge aDUMMY;
}
