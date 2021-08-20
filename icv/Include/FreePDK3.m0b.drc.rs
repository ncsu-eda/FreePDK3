// 3nm FreePDK(TM) M0B ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM0B_1 @= { @ "M0B.1 : Minimum Vertical width of M0B >= 11 nm";
    //Sushant
	internal1( aM0B, < 0.011, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );	
}

rM0B_2 @= { @ "M0B.2 : Minimum Vertical spacing of M0B >= 10 nm";
    //Sushant
	external1(aM0B, < 0.010, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM0B_3 @= { @ "M0B.3 : Minimum Horizontal width of M0B >= 22 nm";
    //Sushant
	internal1( aM0B, < 0.022, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
}

rM0B_4 @= { @ "M0B.4 : M0B minimum horizontal spacing >= 20 nm";
    //Sushant
	external1(aM0B, < 0.020, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM0B_5 @= { @ "M0B.5 : M0B may not bend";
    //Sushant
	not_rectangles(aM0B);	
}

rM0B_6 @= { @ "M0B.6 : Minimum extention of MOB past GCON in horizontal direction >= 3.5 nm";
    //Sushant
	sM0B_VERTICAL_EDGES = angle_edge( aM0B, == 90 );
    sGCON_VERTICAL_EDGES = angle_edge( aGCON, == 90 );
    enclose_error( sGCON_VERTICAL_EDGES, sM0B_VERTICAL_EDGES, < 0.0035, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
}

rM0B_7 @= { @ "M0B.7 : Minimum extention of MOB past V0A >= 6 nm";
    //Sushant
	
}

rM0B_8 @= { @ "M0B.8 : Minimum M0B area >= 242 nm^2";
    //Sushant
	area( aM0B, < 0.000242);
}
