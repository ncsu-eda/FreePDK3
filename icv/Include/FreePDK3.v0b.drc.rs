// 3nm FreePDK(TM) V0B ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV0B_1 @= { @ "V0B.1 : V0B shape is a rectangle";
	//Sushant
     not_rectangles(aV0B);
}

rV0B_2 @= { @ "V0B.2 : V0B exact horizontal width  = 14 nm";
	//Sushant
    gGRV0B2_check = internal1( aV0B, == 0.014, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aV0B not gGRV0B2_check;
}

rV0B_3 @= { @ "V0B.3 : V0B exact vertical width = 10 nm";
	//Sushant
    gGRV0B3_check = internal1( aV0B, == 0.010, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aV0B not gGRV0B3_check;	
}

rV0B_4 @= { @ "V0B.4 : Minimum vertical spacing of V0B - Full alignment >= 10.5 nm";
	//Sushant
    external1(aV0B, < 0.0105, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);   
}

rV0B_5 @= { @ "V0B.5 : Minimum horizontal spacing of V0B - Full alignment >= 20 nm";
	//Sushant
	external1(aV0B, < 0.020, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);    
}

rV0B_6 @= { @ "V0B.6 : Minimum corner-to-corner spacing between two V0B instances >= 22 nm";
	//Sushant
   	external_corner1(aV0B, <0.022); 
}

rV0B_7 @= { @ "V0B.7 : V0B must always interact with M0B and M1";
	//Sushant
    not_interacting( aV0B, aM0B );
    not_interacting( aV0B, aM1 );
}

rV0B_8 @= { @ "V0B.8 : V0B enclosure by M0B on two opposite sides, horizontal direction = 4 nm";
	//Sushant
    //gGRV0B8_check = enclose( aV0B, aM0B, distance == 0.004, direction = HORIZONTAL, extension = NONE );
    //aV0B not gGRV0B8_check;
	sV0B_VERTICAL_EDGES = angle_edge( aV0B, == 90 );
    sM0B_VERTICAL_EDGES = angle_edge( aM0B, == 90 );
    enclose_error( sV0B_VERTICAL_EDGES, sM0B_VERTICAL_EDGES, < 0.004, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
	aV0B not aM0B;

}

rV0B_9 @= { @ "V0B.9 : V0B enclosure by M0B on two opposite sides, VERTICAL direction = 0 nm";
	//Sushant
    //gGRV0B9_check = enclose( aV0B, aM0B, distance == 0, direction = VERTICAL, extension = RECTANGLE );
    //aV0B not gGRV0B9_check;
    sM0B_HORIZONTAL_EDGES = angle_edge( aM0B, == 0 );
	and_edge( sM0B_HORIZONTAL_EDGES, aV0B, false );
}

rV0B_10 @= { @ "V0B.10 : V0B enclosure by M1 on two opposite sides, horizontal direction = 0 nm";
	//Sushant
    //gGRV0B10_check = enclose( aV0B, aM1, distance == 0, direction = HORIZONTAL, extension = RECTANGLE );
    //aV0B not gGRV0B10_check;
	sM1_VERTICAL_EDGES = angle_edge( aM1, == 90 );
	and_edge( sM1_VERTICAL_EDGES, aV0B, false );
}

rV0B_11 @= { @ "V0B.11 : V0B enclosure by M1 on two opposite sides, VERTICAL direction = 2.5 nm";
	//Sushant
    //gGRV0B11_check = enclose( aV0B, aM1, distance == 0.0025, direction = VERTICAL, extension = RECTANGLE );
    //aV0B not gGRV0B11_check;
	sV0B_HORIZONTAL_EDGES = angle_edge( aV0B, == 0 );
    sM1_HORIZONTAL_EDGES = angle_edge( aM1, == 0 );
    enclose_error( sV0B_HORIZONTAL_EDGES, sM1_HORIZONTAL_EDGES, < 0.0025, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
	aV0B not aM1;
}


