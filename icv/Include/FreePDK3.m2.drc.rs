// 3nm FreePDK(TM) M2 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM2_1 @= { @ "M2.1 : METAL2 width minimum >= 14nm";
    //Sushant
	internal1( aM2, < 0.014, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rM2_2 @= { @ "M2.2 : METAL2 spacing minimum >= 14nm";
    //Sushant
	external1(aM2, < 0.014, extension = RADIAL, intersecting = { }, intersection_angle = < 90,  look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

m2_w_015 =wide(aM2, distance >= 0.015); 
m2_w_030 =wide(aM2, distance >= 0.030); 

rM2_3 @= { @ "M2.3 : METAL2 maximum width in vertical direction = 750nm";
    //Sushant	
	m2_w_750 = internal1( aM2, < 0.7505, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aM2 not m2_w_750;
}

m2_w_045 =wide(aM2, distance >= 0.045);
m2_w_135 =wide(aM2, distance >= 0.135);	
m2_w_405 = wide(aM2, distance >= 0.405);

m2_w_045_135 = m2_w_045 not m2_w_135;
m2_w_135_405 = m2_w_135 not m2_w_405;


rM2_4 @= { @ "M2.4 : Minimum spacing of METAL2 wider than 45nm and longer than 45nm >= 45nm";
    //Sushant
    m2_045 = aM2 not m2_w_045_135;
	mLAYER_2_1 = external2(m2_w_045_135,m2_045,distance < 0.045, extension = NONE,projection_length >= 0.045, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_2_2 = external1(m2_w_045_135,distance < 0.045, extension = NONE,projection_length >= 0.045, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_2_1 or mLAYER_2_2;
}

rM2_5 @= { @ "M2.5 : Minimum spacing of METAL2 wider than 135nm and longer than 135nm >= 135nm";
    //Sushant
    m2_135 = aM2 not m2_w_135_405;
	mLAYER_2_3 = external2(m2_w_135_405,m2_135,distance < 0.135, extension = NONE,projection_length >= 0.135, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_2_4 = external1(m2_w_135_405,distance < 0.135, extension = NONE,projection_length >= 0.135, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_2_3 or mLAYER_2_4;	
}

rM2_6 @= { @ "M2.6 : Minimum spacing of METAL2 wider than 405nm and longer than 405nm >= 405nm";
    //Sushant
	mLAYER_2_5 = external2(m2_w_405,aM2,distance < 0.405, extension = NONE,projection_length >= 0.405, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_2_6 = external1(m2_w_405,distance < 0.405, extension = NONE,projection_length >= 0.405, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_2_5 or mLAYER_2_6;	
}

rM2_7 @= { @ "M2.7 : Double patterning Error";
    //Sushant
	m2_Links = external1_error(aM2, < 0.015, extension = RADIAL, intersecting = { }, intersection_angle = < 90,  look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	m2_conflict = two_color(aM2, m2_Links, aM2_color1, aM2_color2, even_loops, pre_color_layer ,output_even_loops = true, output_type=ODD_LOOP_INSIDE_RING, color_preference = BALANCED ); 
	copy(m2_conflict);
	color_output = gds_library("m2_dpt_output.gds");
	write_gds(color_output, layers = {{aM2, {1}}, {aM2_color1, {2}}, {aM2_color2, {3}}, {m2_conflict, {4}}});
}

rM2_8 @= { @ "M2.8 : Density Balancing Rule: The density of decomposed metals should be between 23 and 77%";
	//Sushant 
	density(window_layer = chip_extent(), // The polygon layer that defines the boundaries where layers are processed for density calculations
            layer_hash         = {//"nodes"      => tmp_nodes,
								 "out_color1" => aM2_color1,
								 "out_color2" => aM2_color2,},
            window_function    = color_balance_func_M,
            merge_errors = true );

}