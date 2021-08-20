// 3nm FreePDK(TM) NIMPIM ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rNIMPIM_1 @= { @ "NIM/PIM.1 : Minimum width/spacing/notch of NIM/PIM >= 84 nm";
    //Sushant
	internal1( aNIM, < 0.084, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	internal1( aPIM, < 0.084, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	external1( aNIM, < 0.084, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external1( aNIM, < 0.084, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external1( aPIM, < 0.084, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external1( aPIM, < 0.084, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	
}

rNIMPIM_2 @= { @ "NIM/PIM.2 : Minimum vertical width of NIM/PIM  >= 57.5 nm";
    //Sushant
	internal1( aNIM, < 0.0575, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	internal1( aPIM, < 0.0575, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rNIMPIM_5 @= { @ "NIM/PIM.5 : Minimum enlose of ACT by NIM/PIM on vertical direction >= 20 nm";
	gNIMPIM = or(aNIM, aPIM);
    	not_enclosed_by( aACT, gNIMPIM, distances = { { { 0.0135, NONE }, { 0.020, NONE }, { 0.0135, NONE }, { 0.020, NONE } } }, intersecting_failures = {     ACUTE, POINT_TOUCH, TOUCH }, not_inside = FAIL );    
}

rNIMPIM_6 @= { @ "NIM/PIM.6 : Minimum enlose of ACT by NIM/PIM on horizontal direction >= 13.5 nm";
	gNIMPIM = or(aNIM, aPIM);
    	not_enclosed_by( aACT, gNIMPIM, distances = { { { 0.0135, NONE }, { 0.020, NONE }, { 0.0135, NONE }, { 0.020, NONE } } }, intersecting_failures = {     ACUTE, POINT_TOUCH, TOUCH }, not_inside = FAIL );	    
}

rNIMPIM_7 @= { @ "NIM/PIM.7 : Minimum NIM/PIM area/enclosed area >= 4830 nm^2";
    //Sushant
	area( aPIM, < 0.004830);
	area( aNIM, < 0.004830);
}


rNIMPIM_10 @= { @ "NIM/PIM.10 : NIM and PIM may not overlap ";
    //Sushant
	interacting(aNIM, aPIM, include_touch = NONE);
}
