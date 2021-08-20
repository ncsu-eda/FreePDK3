// 3nm FreePDK(TM) BPR ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rBPR_1 @= { @ "BRP.1 : BPR vertical width = 31.5 nm";
   gGRBPR1_check = internal1( aBPR, == 0.0315, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH ); 
	aBPR not gGRBPR1_check;
}

rBPR_2 @= { @ "BPR.2 : Minimum vertical spacing between BPR layers >= 84 nm";
// TBD
	external1(aBPR, < 0.084, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 

}

rBPR_3 @= { @ "BPR.3 : BPR must be continuous";
// TBD
	external1(aBPR, < 1, direction = HORIZONTAL, extension = NONE);
}

rBPR_4 @= { @ "BPR.4 : BPR may not bend";
// Sushant
	not_rectangles(aBPR);
}

