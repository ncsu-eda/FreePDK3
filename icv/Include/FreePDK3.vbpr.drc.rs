// 3nm FreePDK(TM) VBPR ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rVBPR_1 @= { @ "VBPR.1 : VBPR must be rectangle";
    not_rectangles(aVBPR);
}

rVBPR_2 @= { @ "VBPR.2 : Exact vertical width of VBPR = 10.5 nm";
// Sushant
	gGRVBPR2_check = internal1( aVBPR, == 0.0105, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aVBPR not gGRVBPR2_check;
}

rVBPR_3 @= { @ "VBPR.3 : Exact horizontal length of VBPR = 15 nm";
    gGRVBPR3_check = internal1( aVBPR, == 0.015, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aVBPR not gGRVBPR3_check;
}

rVBPR_4 @= { @ "VBPR.4 : VBPR enclosure by BPR on two opposite sides, horizontal direction = 13.5 nm";
//    gGRVBPR4_chk = enclose( aVBPR, aBPR, distance == 13.5, direction = HORIZONTAL );
//    aVBPR not gGRVBPR4_chk;
	sVBPR_VERTICAL_EDGES = angle_edge( aVBPR, == 90 );
    sBPR_VERTICAL_EDGES = angle_edge( aBPR, == 90 );
    enclose_error( sVBPR_VERTICAL_EDGES, sBPR_VERTICAL_EDGES, < 0.0135, edge_containment = INSIDE_TO_OUTSIDE, extension = RADIAL, intersecting = {  }, look_thru = NOT_ADJACENT );
}

rVBPR_5 @= { @ "VBPR.5 : VBPR enclosure by M0A on two opposite sides, vertical direction = 0 nm";
//    gGRVBPR5_chk = enclose( aVBPR, aM0A, distance == 0, direction = VERTICAL );
//    aVBPR not gGRVBPR5_chk;
	sM0A_HORIZONTAL_EDGES = angle_edge( aM0A, == 0 );
	and_edge( sM0A_HORIZONTAL_EDGES, aVBPR, false );
	aVBPR not aM0A;
}

rVBPR_6 @= { @ "VBPR.6 : Minimum horizontal spacing between two VBPR layers >= 27 nm";
// Sushant
	external1(aVBPR, < 0.027, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = HORIZONTAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rVBPR_7 @= { @ "VBPR.7 : Minimum vertical spacing betwenn two VBPR layers >= 10 nm";
// Sushant 
	external1(aVBPR, < 0.010, extension = RADIAL, intersecting = { }, intersection_angle = < 90, direction = VERTICAL, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);   
}

rVBPR_8 @= { @ "VBPR.8 : Minmum spacing between VBPR and ACTIVE layer polygons not on the same net >= 10 nm";
    external2( aVBPR, aACT, < 0.010, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);
}

rVBPR_9 @= { @ "VBPR.9 : VBPR may not interact with GCUT or GATE or DUMMY";
    gGRVBPR9_check = or(aGCUT, aGATE);
    interacting( aVBPR, gGRVBPR9_check, include_touch = NONE);
    interacting( aVBPR, aDUMMY, include_touch = NONE);
}
