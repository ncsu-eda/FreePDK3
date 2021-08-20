// 3nm FreePDK(TM) M10 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM10_1 @= { @ "M10.1 : METAL10 width minimum >= 80nm";
    //Sushant
	internal1( aM10, < 0.08, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rM10_2 @= { @ "M10.2 : METAL10 spacing minimum >= 80nm";
    //Sushant
	external1(aM10, < 0.08, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM10_3 @= { @ "M10.3 : METAL10 maximum width in vertical direction = 4000nm";
    //Sushant
    m10_w_4000 = internal1( aM10, < 4.0005, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aM10 not m10_w_4000;	
}

m10_w_240 =wide(aM10, distance >= 0.240);
m10_w_720 =wide(aM10, distance >= 0.720);	
m10_w_2160 = wide(aM10, distance >= 2.160);

m10_w_240_720 = m10_w_240 not m10_w_720;
m10_w_720_2160 = m10_w_720 not m10_w_2160;


rM10_4 @= { @ "M10.4 : Minimum spacing of METAL10 wider than 240nm and longer than 240nm >= 240nm";
    //Sushant
    m10_240 = aM10 not m10_w_240_720;
	mLAYER_10_1 = external2(m10_w_240_720,m10_240,distance < 0.240, extension = NONE,projection_length >= 0.240, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_10_2 = external1(m10_w_240_720,distance < 0.240, extension = NONE,projection_length >= 0.240, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_10_1 or mLAYER_10_2;
}

rM10_5 @= { @ "M10.5 : Minimum spacing of METAL10 wider than 720nm and longer than 720nm >= 720nm";
    //Sushant
    m10_720 = aM10 not m10_w_720_2160;
	mLAYER_10_3 = external2(m10_w_720_2160,m10_720,distance < 0.720, extension = NONE,projection_length >= 0.720, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_10_4 = external1(m10_w_720_2160,distance < 0.720, extension = NONE,projection_length >= 0.720, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_10_3 or mLAYER_10_4;	
}

rM10_6 @= { @ "M10.6 : Minimum spacing of METAL10 wider than 2160nm and longer than 2160nm >= 2160nm";
    //Sushant
	mLAYER_10_5 = external2(m10_w_2160,aM10,distance < 2.160, extension = NONE,projection_length >= 2.160, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_10_6 = external1(m10_w_2160,distance < 2.160, extension = NONE,projection_length >= 2.160, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_10_5 or mLAYER_10_6;	
}