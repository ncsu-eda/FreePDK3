// 3nm FreePDK(TM) M6 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM6_1 @= { @ "M6.1 : METAL6 width minimum >= 24nm";
    //Sushant
	internal1( aM6, < 0.024, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rM6_2 @= { @ "M6.2 : METAL6 spacing minimum >= 24nm";
    //Sushant
	external1(aM6, < 0.024, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM6_3 @= { @ "M6.3 : METAL6 maximum width in vertical direction = 1200nm";
    //Sushant
    m6_w_1200 = internal1( aM6, < 1.2005, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aM6 not m6_w_1200;	
}

m6_w_072 =wide(aM6, distance >= 0.072);
m6_w_216 =wide(aM6, distance >= 0.216);	
m6_w_648 = wide(aM6, distance >= 0.648);

m6_w_072_216 = m6_w_072 not m6_w_216;
m6_w_216_648 = m6_w_216 not m6_w_648;


rM6_4 @= { @ "M6.4 : Minimum spacing of METAL6 wider than 72nm and longer than 72nm >= 72nm";
    //Sushant
    m6_072 = aM6 not m6_w_072_216;
	mLAYER_6_1 = external2(m6_w_072_216,m6_072,distance < 0.072, extension = NONE,projection_length >= 0.072, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_6_2 = external1(m6_w_072_216,distance < 0.072, extension = NONE,projection_length >= 0.072, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_6_1 or mLAYER_6_2;
}

rM6_5 @= { @ "M6.5 : Minimum spacing of METAL6 wider than 216nm and longer than 216nm >= 216nm";
    //Sushant
    m6_216 = aM6 not m6_w_216_648;
	mLAYER_6_3 = external2(m6_w_216_648,m6_216,distance < 0.216, extension = NONE,projection_length >= 0.216, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_6_4 = external1(m6_w_216_648,distance < 0.216, extension = NONE,projection_length >= 0.216, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_6_3 or mLAYER_6_4;	
}

rM6_6 @= { @ "M6.6 : Minimum spacing of METAL6 wider than 648nm and longer than 648nm >= 648nm";
    //Sushant
	mLAYER_6_5 = external2(m6_w_648,aM6,distance < 0.648, extension = NONE,projection_length >= 0.648, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_6_6 = external1(m6_w_648,distance < 0.648, extension = NONE,projection_length >= 0.648, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_6_5 or mLAYER_6_6;	
}