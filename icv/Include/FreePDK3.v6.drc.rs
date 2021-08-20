// 3nm FreePDK(TM) V6 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV6_1 @= { @ "V6.1 : V6 shape is rectangle";
	//Sushant
    not_rectangles(aV6); 
}

rV6_2 @= { @ "V6.2 : V6 is a rectangle with 40nm horizontal edge and 24nm vertical edge";
	//Sushant
    gGRV6_1_check = internal1( aV6, == 0.04, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aV6 not gGRV6_1_check;
    gGRV6_2_check = internal1( aV6, == 0.024, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aV6 not gGRV6_2_check;
}

rV6_3a @= { @ "V6.3a : Minimum horizontal spacing of V6 - Full alignment >= 40nm";
	//Sushant
    external1(aV6, < 0.04, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);
}

rV6_3b @= { @ "V6.3b : Minimum vertical spacing of V6 - Full alignment >= 24nm";
	//Sushant
    external1(aV6, < 0.024, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rV6_4 @= { @ "V6.4 : V6 should be enclosed between M6 and M7";
	//Sushant
    not_interacting( aV6, aM6 );
    not_interacting( aV6, aM7 );
}

rV6_5 @= { @ "V6.5 : V6 enclosure by M6 on two opposite sides, horizontal direction >= 10nm";
	//Sushant
    sV6_VERTICAL_EDGES = angle_edge( aV6, == 90 );
    sM6_VERTICAL_EDGES = angle_edge( aM6, == 90 );
    enclose_error( sV6_VERTICAL_EDGES, sM6_VERTICAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV6 not aM6; 
}

rV6_6 @= { @ "V6.6 : V6 enclosure by M6 on two opposite sides, vertical direction = 0nm";
	//Sushant
    sM6_HORIZONTAL_EDGES = angle_edge( aM6, == 0 );
	and_edge( sM6_HORIZONTAL_EDGES, aV6, false ); 
}

rV6_7 @= { @ "V6.7 : V6 enclosure by M7 on two opposite sides, horizontal direction = 0nm";
	//Sushant
    sM7_VERTICAL_EDGES = angle_edge( aM7, == 90 );
	and_edge( sM7_VERTICAL_EDGES, aV6, false );
}

rV6_8 @= { @ "V6.8 : V6 enclosure by M7 on two opposite sides, vertical direction >= 10nm";
	//Sushant
    sV6_HORIZONTAL_EDGES = angle_edge( aV6, == 0 );
    sM7_HORIZONTAL_EDGES = angle_edge( aM7, == 0 );
    enclose_error( sV6_HORIZONTAL_EDGES, sM7_HORIZONTAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV6 not aM7; 
}

rV6_9 @= { @ "V6.9 : Minimum area overlap between M6 and V6 >= 960 nm^2";
	//Sushant
    gV10 = and(aM6, aV6);
	area(gV10, < 0.000960);  
}
