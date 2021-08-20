// 3nm FreePDK(TM) RDL ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rMRDL_1 @= { @ "MRDL.1 : RDL width minimum >= 1.6um";
    //Sushant
	internal1( aRDL, < 1.6, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rMRDL_2 @= { @ "MRDL.2 : RDL spacing minimum >= 1.6um";
    //Sushant
	external1(aRDL, < 1.6, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}