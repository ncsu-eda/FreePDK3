// 3nm FreePDK(TM) M5 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM5_1 @= { @ "M5.1 : METAL5 width minimum >= 24nm";
    //Sushant
	internal1( aM5, < 0.024, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
}

rM5_2 @= { @ "M5.2 : METAL5 spacing minimum >= 24nm";
    //Sushant
	external1(aM5, < 0.024, extension = RADIAL, intersecting = { }, intersection_angle = < 90,  look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM5_3 @= { @ "M5.3 : METAL5 maximum width in horizontal direction= 1200nm";
    //Sushant
    m5_w_1200 = internal1( aM5, < 1.2005, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aM5 not m5_w_1200;	
}

m5_w_072 =wide(aM5, distance >= 0.072);
m5_w_216 =wide(aM5, distance >= 0.216);	
m5_w_648 = wide(aM5, distance >= 0.648);

m5_w_072_216 = m5_w_072 not m5_w_216;
m5_w_216_648 = m5_w_216 not m5_w_648;


rM5_4 @= { @ "M5.4 : Minimum spacing of METAL5 wider than 72nm and longer than 72nm >= 72nm";
    //Sushant
    m5_072 = aM5 not m5_w_072_216;
	mLAYER_5_1 = external2(m5_w_072_216,m5_072,distance < 0.072, extension = NONE,projection_length >= 0.072, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_5_2 = external1(m5_w_072_216,distance < 0.072, extension = NONE,projection_length >= 0.072, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_5_1 or mLAYER_5_2;
}

rM5_5 @= { @ "M5.5 : Minimum spacing of METAL5 wider than 216nm and longer than 216nm >= 216nm";
    //Sushant
    m5_216 = aM5 not m5_w_216_648;
	mLAYER_5_3 = external2(m5_w_216_648,m5_216,distance < 0.216, extension = NONE,projection_length >= 0.216, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_5_4 = external1(m5_w_216_648,distance < 0.216, extension = NONE,projection_length >= 0.216, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_5_3 or mLAYER_5_4;	
}

rM5_6 @= { @ "M5.6 : Minimum spacing of METAL5 wider than 648nm and longer than 648nm >= 648nm";
    //Sushant
	mLAYER_5_5 = external2(m5_w_648,aM5,distance < 0.648, extension = NONE,projection_length >= 0.648, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_5_6 = external1(m5_w_648,distance < 0.648, extension = NONE,projection_length >= 0.648, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_5_5 or mLAYER_5_6;	
}