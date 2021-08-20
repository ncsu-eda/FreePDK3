// 3nm FreePDK(TM) V11 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV11_1 @= { @ "V11.1 : V11 shape is rectangle";
	//Sushant
    not_rectangles(aV11);
}

rV11_2 @= { @ "V11.2 : V11 is a rectangle with 80nm horizontal edge and 160nm vertical edge";
	//Sushant
    gGRV11_1_check = internal1( aV11, == 0.08, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aV11 not gGRV11_1_check;
    gGRV11_2_check = internal1( aV11, == 0.160, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aV11 not gGRV11_2_check;
}

rV11_3a @= { @ "V11.3a : Minimum horizontal spacing of V11 - Full alignment >= 80nm";
	//Sushant
    external1(aV11, < 0.08, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rV11_3b @= { @ "V11.3b : Minimum vertical spacing of V11 - Full alignment >= 160nm";
	//Sushant
    external1(aV11, < 0.160, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rV11_4 @= { @ "V11.4 : V11 should be enclosed between M11 and M12";
	//Sushant
    not_interacting( aV11, aM11 );
    not_interacting( aV11, aM12 );
}

rV11_5 @= { @ "V11.5 : V11 enclosure by M11 on two opposite sides, horizontal direction = 0nm";
	//Sushant
    // sM11_VERTICAL_EDGES = angle_edge( aM11, == 90 );
	// and_edge( sM11_VERTICAL_EDGES, aV11, false );
	aV11 not aM11;
}

rV11_6 @= { @ "V11.6 : V11 enclosure by M11 on two opposite sides, vertical direction >= 10nm";
	//Sushant
    sV11_HORIZONTAL_EDGES = angle_edge( aV11, == 0 );
    sM11_HORIZONTAL_EDGES = angle_edge( aM11, == 0 );
    enclose_error( sV11_HORIZONTAL_EDGES, sM11_HORIZONTAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV11 not aM11;
}

rV11_7 @= { @ "V11.7 : V11 enclosure by M12 on two opposite sides, horizontal direction >= 10nm";
	//Sushant
    sV11_VERTICAL_EDGES = angle_edge( aV11, == 90 );
    sM12_VERTICAL_EDGES = angle_edge( aM12, == 90 );
    enclose_error( sV11_VERTICAL_EDGES, sM12_VERTICAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV11 not aM12;
}

rV11_8 @= { @ "V11.8 : V11 enclosure by M12 on two opposite sides, vertical direction = 0nm";
	//Sushant
    // sM12_HORIZONTAL_EDGES = angle_edge( aM12, == 90 );
	// and_edge( sM12_HORIZONTAL_EDGES, aV11, false );
	aV11 not aM12;
}

rV11_9 @= { @ "V11.9 : Minimum area overlap between M11 and V11 >= 12800nm^2";
	//Sushant
    gV10 = and(aM11, aV11);
	area(gV10, < 0.012800); 
}