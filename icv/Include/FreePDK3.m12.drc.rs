// 3nm FreePDK(TM) M12 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM12_1 @= { @ "M12.1 : METAL12 width minimum >= 160nm";
    //Sushant
	internal1( aM12, < 0.16, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rM12_2 @= { @ "M12.2 : METAL12 spacing minimum >= 160nm";
    //Sushant
	external1(aM12, < 0.16, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM12_3 @= { @ "M12.3 : METAL12 maximum width in vertical direction = 8000nm";
    //Sushant
    m12_w_8000 = internal1( aM12, < 8.0005, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aM12 not m12_w_8000;	
}

m12_w_480 =wide(aM12, distance >= 0.480);
m12_w_1440 =wide(aM12, distance >= 1.440);	
m12_w_4320 = wide(aM12, distance >= 4.320);

m12_w_480_1440 = m12_w_480 not m12_w_1440;
m12_w_1440_4320 = m12_w_1440 not m12_w_4320;


rM12_4 @= { @ "M12.4 : Minimum spacing of METAL12 wider than 480nm and longer than 480nm >= 480nm";
    //Sushant
    m12_480 = aM12 not m12_w_480_1440;
	mLAYER_12_1 = external2(m12_w_480_1440,m12_480,distance < 0.480, extension = NONE,projection_length >= 0.480, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_12_2 = external1(m12_w_480_1440,distance < 0.480, extension = NONE,projection_length >= 0.480, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_12_1 or mLAYER_12_2;
}

rM12_5 @= { @ "M12.5 : Minimum spacing of METAL12 wider than 1440nm and longer than 1440nm >= 1440nm";
    //Sushant
    m12_1440 = aM12 not m12_w_1440_4320;
	mLAYER_12_3 = external2(m12_w_1440_4320,m12_1440,distance < 1.440, extension = NONE,projection_length >= 1.440, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_12_4 = external1(m12_w_1440_4320,distance < 1.440, extension = NONE,projection_length >= 1.440, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_12_3 or mLAYER_12_4;	
}

rM12_6 @= { @ "M12.6 : Minimum spacing of METAL12 wider than 4320nm and longer than 4320nm >= 4320nm";
    //Sushant
	mLAYER_12_5 = external2(m12_w_4320,aM12,distance < 4.320, extension = NONE,projection_length >= 4.320, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_12_6 = external1(m12_w_4320,distance < 4.320, extension = NONE,projection_length >= 4.320, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_12_5 or mLAYER_12_6;	
}