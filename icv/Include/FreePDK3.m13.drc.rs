// 3nm FreePDK(TM) M13 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM13_1 @= { @ "M13.1 : METAL13 width minimum >= 160nm";
    //Sushant
	internal1( aM13, < 0.16, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
}

rM13_2 @= { @ "M13.2 : METAL13 spacing minimum >= 160nm";
    //Sushant
	external1(aM13, < 0.16, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM13_3 @= { @ "M13.3 : METAL13 maximum width in horizontal direction = 8000nm";
    //Sushant
    m13_w_8000 = internal1( aM13, < 8.0005, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aM13 not m13_w_8000;
}

m13_w_480 =wide(aM13, distance >= 0.480);
m13_w_1440 =wide(aM13, distance >= 1.440);	
m13_w_4320 = wide(aM13, distance >= 4.320);

m13_w_480_1440 = m13_w_480 not m13_w_1440;
m13_w_1440_4320 = m13_w_1440 not m13_w_4320;


rM13_4 @= { @ "M13.4 : Minimum spacing of METAL13 wider than 480nm and longer than 480nm >= 480nm";
    //Sushant
    m13_480 = aM13 not m13_w_480_1440;
	mLAYER_13_1 = external2(m13_w_480_1440,m13_480,distance < 0.480, extension = NONE,projection_length >= 0.480, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_13_2 = external1(m13_w_480_1440,distance < 0.480, extension = NONE,projection_length >= 0.480, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_13_1 or mLAYER_13_2;
}

rM13_5 @= { @ "M13.5 : Minimum spacing of METAL13 wider than 1440nm and longer than 1440nm >= 1440nm";
    //Sushant
    m13_1440 = aM13 not m13_w_1440_4320;
	mLAYER_13_3 = external2(m13_w_1440_4320,m13_1440,distance < 1.440, extension = NONE,projection_length >= 1.440, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_13_4 = external1(m13_w_1440_4320,distance < 1.440, extension = NONE,projection_length >= 1.440, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_13_3 or mLAYER_13_4;	
}

rM13_6 @= { @ "M13.6 : Minimum spacing of METAL13 wider than 4320nm and longer than 4320nm >= 4320nm";
    //Sushant
	mLAYER_13_5 = external2(m13_w_4320,aM13,distance < 4.320, extension = NONE,projection_length >= 4.320, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_13_6 = external1(m13_w_4320,distance < 4.320, extension = NONE,projection_length >= 4.320, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_13_5 or mLAYER_13_6;	
}