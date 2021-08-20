// 3nm FreePDK(TM) M11 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM11_1 @= { @ "M11.1 : METAL11 width minimum >= 80nm";
    //Sushant
	internal1( aM11, < 0.08, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
}

rM11_2 @= { @ "M11.2 : METAL11 spacing minimum >= 80nm";
    //Sushant
	external1(aM11, < 0.08, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM11_3 @= { @ "M11.3 : METAL11 maximum width in horizontal direction = 4000nm";
    //Sushant
    m11_w_4000 = internal1( aM11, < 4.0005, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aM11 not m11_w_4000;
}

m11_w_240 =wide(aM11, distance >= 0.240);
m11_w_720 =wide(aM11, distance >= 0.720);	
m11_w_2160 = wide(aM11, distance >= 2.160);

m11_w_240_720 = m11_w_240 not m11_w_720;
m11_w_720_2160 = m11_w_720 not m11_w_2160;


rM11_4 @= { @ "M11.4 : Minimum spacing of METAL11 wider than 240nm and longer than 240nm >= 240nm";
    //Sushant
    m11_240 = aM11 not m11_w_240_720;
	mLAYER_11_1 = external2(m11_w_240_720,m11_240,distance < 0.240, extension = NONE,projection_length >= 0.240, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_11_2 = external1(m11_w_240_720,distance < 0.240, extension = NONE,projection_length >= 0.240, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_11_1 or mLAYER_11_2;
}

rM11_5 @= { @ "M11.5 : Minimum spacing of METAL11 wider than 720nm and longer than 720nm >= 720nm";
    //Sushant
    m11_720 = aM11 not m11_w_720_2160;
	mLAYER_11_3 = external2(m11_w_720_2160,m11_720,distance < 0.720, extension = NONE,projection_length >= 0.720, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_11_4 = external1(m11_w_720_2160,distance < 0.720, extension = NONE,projection_length >= 0.720, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_11_3 or mLAYER_11_4;	
}

rM11_6 @= { @ "M11.6 : Minimum spacing of METAL11 wider than 2160nm and longer than 2160nm >= 2160nm";
    //Sushant
	mLAYER_11_5 = external2(m11_w_2160,aM11,distance < 2.160, extension = NONE,projection_length >= 2.160, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_11_6 = external1(m11_w_2160,distance < 2.160, extension = NONE,projection_length >= 2.160, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_11_5 or mLAYER_11_6;	
}