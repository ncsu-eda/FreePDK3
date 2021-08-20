// 3nm FreePDK(TM) V12 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rV12_1 @= { @ "V12.1 : V12 shape is square";
	//Sushant
    not_aspect_ratio( aV12, direction = SHORT_BY_LONG, orientation = ORTHOGONAL, ratio = == 1 );
}

rV12_2 @= { @ "V12.2 : V12 is a square with 160nm edge length";
	//Sushant
    aspect_ratio( rectangles( aV12, sides = { != 0.16, != 0.16 } ), direction = SHORT_BY_LONG, ratio = == 1 );
}

rV12_3 @= { @ "V12.3 : Minimum spacing of V12 - Full alignment >= 160nm";
	//Sushant
    external1(aV12, < 0.16, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	external1(aV12, < 0.16, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);
}

rV12_4 @= { @ "V12.4 : V12 should be enclosed between M12 and M13";
	//Sushant
    not_interacting( aV12, aM12 );
    not_interacting( aV12, aM13 );
}

rV12_5 @= { @ "V12.5 : V12 enclosure by M12 on two opposite sides, horizontal direction >= 10nm";
	//Sushant
    sV12_VERTICAL_EDGES = angle_edge( aV12, == 90 );
    sM12_VERTICAL_EDGES = angle_edge( aM12, == 90 );
    enclose_error( sV12_VERTICAL_EDGES, sM12_VERTICAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV12 not aM12;
}

rV12_6 @= { @ "V12.6 : V12 enclosure by M12 on two opposite sides, vertical direction = 0nm";
	//Sushant
    // sM12_HORIZONTAL_EDGES = angle_edge( aM12, == 0 );
	// and_edge( sM12_HORIZONTAL_EDGES, aV12, false ); 
	aV12 not aM12;
}

rV12_7 @= { @ "V12.7 : V12 enclosure by M13 on two opposite sides, horizontal direction = 0nm";
	//Sushant
    // sM13_VERTICAL_EDGES = angle_edge( aM13, == 90 );
	// and_edge( sM13_VERTICAL_EDGES, aV12, false );
	aV12 not aM13;
}

rV12_8 @= { @ "V12.8 : V12 enclosure by M13 on two opposite sides, vertical direction >= 10nm";
	//Sushant
    sV12_HORIZONTAL_EDGES = angle_edge( aV12, == 0 );
    sM13_HORIZONTAL_EDGES = angle_edge( aM13, == 0 );
    enclose_error( sV12_HORIZONTAL_EDGES, sM13_HORIZONTAL_EDGES, < 0.01, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
    aV12 not aM13; 
}

rV12_9 @= { @ "V12.9 : Minimum area overlap between M12 and V12 >= 25600nm^2";
	//Sushant
    gV12 = and(aM12, aV12);
	area(gV12, < 0.025600);
}