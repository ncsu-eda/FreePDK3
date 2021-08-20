// 3nm FreePDK(TM) NW ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rNW_1 @= { @ "NW.1 : Minimum vertical width of NW >= 57.5 nm";
    internal1( aNW, < 0.0575, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rNW_2 @= { @ "NW.2 : Minimum horizontal width of NW >= 84 nm";
   internal1( aNW, < 0.084, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
}

rNW_3 @= { @ "NW.3 : Minimum area/enclosed area of NW >= 6237 nm^2";
    area( aNW, < 0.006237);
}

rNW_4 @= { @ "NW.4 : NW must be orthogonal";
    angle_edge( aNW, ( 0, 90 ) );
}

rNW_5 @= { @ "NW.5 : Minimum extension of NW past GATE (not cut by GCUT) >= 7 nm";
	//and(aGCUT, aGATE);
   enclose( aGATE, aNW, < 0.007, extension = NONE, direction = VERTICAL);
}
