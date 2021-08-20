// 3nm FreePDK(TM) M3 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM3_1 @= { @ "M3.1 : METAL3 width minimum >= 15nm";
    //Sushant
	internal1( aM3, < 0.015, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
}

rM3_2 @= { @ "M3.2 : METAL3 spacing minimum >= 15nm";
    //Sushant
	external1(aM3, < 0.015, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM3_3 @= { @ "M3.3 : METAL3 maximum width in horizontal direction = 750nm";
    //Sushant	
	m3_w_750 = internal1( aM3, < 0.7505, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aM3 not m3_w_750;	
}

m3_w_045 =wide(aM3, distance >= 0.045);
m3_w_135 =wide(aM3, distance >= 0.135);	
m3_w_405 = wide(aM3, distance >= 0.405);

m3_w_045_135 = m3_w_045 not m3_w_135;
m3_w_135_405 = m3_w_135 not m3_w_405;


rM3_4 @= { @ "M3.4 : Minimum spacing of METAL3 wider than 45nm and longer than 45nm >= 45nm";
    //Sushant
    m3_045 = aM3 not m3_w_045_135;
	mLAYER_3_1 = external2(m3_w_045_135,m3_045,distance < 0.045, extension = NONE,projection_length >= 0.045, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_3_2 = external1(m3_w_045_135,distance < 0.045, extension = NONE,projection_length >= 0.045, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_3_1 or mLAYER_3_2;
}

rM3_5 @= { @ "M3.5 : Minimum spacing of METAL3 wider than 135nm and longer than 135nm >= 135nm";
    //Sushant
    m3_135 = aM3 not m3_w_135_405;
	mLAYER_3_3 = external2(m3_w_135_405,m3_135,distance < 0.135, extension = NONE,projection_length >= 0.135, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_3_4 = external1(m3_w_135_405,distance < 0.135, extension = NONE,projection_length >= 0.135, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_3_3 or mLAYER_3_4;	
}

rM3_6 @= { @ "M3.6 : Minimum spacing of METAL3 wider than 405nm and longer than 405nm >= 405nm";
    //Sushant
	mLAYER_3_5 = external2(m3_w_405,aM3,distance < 0.405, extension = NONE,projection_length >= 0.405, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_3_6 = external1(m3_w_405,distance < 0.405, extension = NONE,projection_length >= 0.405, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_3_5 or mLAYER_3_6;	
}

rM3_7 @= { @ "M3.7 : Double patterning Error";
    //Sushant
	
	m3_Links =  external1_error(aM3, < 0.015, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	m3_conflict = two_color(aM3, m3_Links, aM3_color1, aM3_color2, even_loops, pre_color_layer ,output_even_loops = true, output_type=ODD_LOOP_INSIDE_RING, color_preference = BALANCED ); 
	copy(m3_conflict);
	color_output = gds_library("m3_dpt_output.gds");
	write_gds(color_output, layers = {{aM3, {1}}, {aM3_color1, {2}}, {aM3_color2, {3}}, {m3_conflict, {4}}});
}

rM3_8 @= { @ "M3.8 : Density Balancing Rule: The density of decomposed metals should be between 23 and 77%";
	//Sushant 
	density(window_layer = chip_extent(), // The polygon layer that defines the boundaries where layers are processed for density calculations
            layer_hash         = {//"nodes"      => tmp_nodes,
								 "out_color1" => aM3_color1,
								 "out_color2" => aM3_color2,},
            window_function    = color_balance_func_M,
            merge_errors = true );

}