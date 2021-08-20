// 3nm FreePDK(TM) M4 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM4_1 @= { @ "M4.1 : METAL4 width minimum >= 24nm";
    //Sushant
	internal1( aM4, < 0.024, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rM4_2 @= { @ "M4.2 : METAL4 spacing minimum >= 24nm";
    //Sushant
	external1(aM4, < 0.024, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM4_3 @= { @ "M4.3 : METAL4 maximum width in vertical direction = 1200nm";
    //Sushant
    m4_w_1200 = internal1( aM4, < 1.2005, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aM4 not m4_w_1200;	
}

m4_w_072 =wide(aM4, distance >= 0.072);
m4_w_216 =wide(aM4, distance >= 0.216);	
m4_w_648 = wide(aM4, distance >= 0.648);

m4_w_072_216 = m4_w_072 not m4_w_216;
m4_w_216_648 = m4_w_216 not m4_w_648;


rM4_4 @= { @ "M4.4 : Minimum spacing of METAL4 wider than 72nm and longer than 72nm >= 72nm";
    //Sushant
    m4_072 = aM4 not m4_w_072_216;
	mLAYER_4_1 = external2(m4_w_072_216,m4_072,distance < 0.072, extension = NONE,projection_length >= 0.072, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_4_2 = external1(m4_w_072_216,distance < 0.072, extension = NONE,projection_length >= 0.072, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_4_1 or mLAYER_4_2;
}

rM4_5 @= { @ "M4.5 : Minimum spacing of METAL4 wider than 216nm and longer than 216nm >= 216nm";
    //Sushant
    m4_216 = aM4 not m4_w_216_648;
	mLAYER_4_3 = external2(m4_w_216_648,m4_216,distance < 0.216, extension = NONE,projection_length >= 0.216, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_4_4 = external1(m4_w_216_648,distance < 0.216, extension = NONE,projection_length >= 0.216, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_4_3 or mLAYER_4_4;	
}

rM4_6 @= { @ "M4.6 : Minimum spacing of METAL4 wider than 648nm and longer than 648nm >= 648nm";
    //Sushant
	mLAYER_4_5 = external2(m4_w_648,aM4,distance < 0.648, extension = NONE,projection_length >= 0.648, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_4_6 = external1(m4_w_648,distance < 0.648, extension = NONE,projection_length >= 0.648, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_4_5 or mLAYER_4_6;	
}