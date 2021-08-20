// 3nm FreePDK(TM) M8 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM8_1 @= { @ "M8.1 : METAL8 width minimum >= 40nm";
    //Sushant
	internal1( aM8, < 0.04, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
}

rM8_2 @= { @ "M8.2 : METAL8 spacing minimum >= 40nm";
    //Sushant
	external1(aM8, < 0.04, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM8_3 @= { @ "M8.3 : METAL8 maximum width in vertical direction = 2000nm";
    //Sushant
    m8_w_2000 = internal1( aM8, < 2.0005, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = VERTICAL, orthogonal = BOTH );
	aM8 not m8_w_2000;
}

m8_w_120 =wide(aM8, distance >= 0.120);
m8_w_360 =wide(aM8, distance >= 0.360);	
m8_w_1080 = wide(aM8, distance >= 1.080);

m8_w_120_360 = m8_w_120 not m8_w_360;
m8_w_360_1080 = m8_w_360 not m8_w_1080;


rM8_4 @= { @ "M8.4 : Minimum spacing of METAL8 wider than 120nm and longer than 120nm >= 120nm";
    //Sushant
    m8_120 = aM8 not m8_w_120_360;
	mLAYER_8_1 = external2(m8_w_120_360,m8_120,distance < 0.120, extension = NONE,projection_length >= 0.120, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_8_2 = external1(m8_w_120_360,distance < 0.120, extension = NONE,projection_length >= 0.120, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_8_1 or mLAYER_8_2;
}

rM8_5 @= { @ "M8.5 : Minimum spacing of METAL8 wider than 360nm and longer than 360nm >= 360nm";
    //Sushant
    m8_360 = aM8 not m8_w_360_1080;
	mLAYER_8_3 = external2(m8_w_360_1080,m8_360,distance < 0.360, extension = NONE,projection_length >= 0.360, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_8_4 = external1(m8_w_360_1080,distance < 0.360, extension = NONE,projection_length >= 0.360, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_8_3 or mLAYER_8_4;	
}

rM8_6 @= { @ "M8.6 : Minimum spacing of METAL8 wider than 1080nm and longer than 1080nm >= 1080nm";
    //Sushant
	mLAYER_8_5 = external2(m8_w_1080,aM8,distance < 1.080, extension = NONE,projection_length >= 1.080, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_8_6 = external1(m8_w_1080,distance < 1.080, extension = NONE,projection_length >= 1.080, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_8_5 or mLAYER_8_6;	
}

rM8_7 @= { @ "M8.7 : Double patterning Error";
    //Sushant
	
	m8_Links =  external1_error(aM8, < 0.04, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);  
	m8_conflict = two_color(aM8, m8_Links, aM8_color1, aM8_color2, even_loops, pre_color_layer ,output_even_loops = true, output_type=ODD_LOOP_INSIDE_RING, color_preference = BALANCED ); 
	copy(m8_conflict);
	color_output = gds_library("m8_dpt_output.gds");
	write_gds(color_output, layers = {{aM8, {1}}, {aM8_color1, {2}}, {aM8_color2, {3}}, {m8_conflict, {4}}});
	
	// aM8_color1 = color1_out;
	// aM8_color2 = color2_out;
}

rM8_8 @= { @ "M8.8 : Density Balancing Rule: The density of decomposed metals should be between 23 and 77%";
	//Sushant 
	density(window_layer = chip_extent(), // The polygon layer that defines the boundaries where layers are processed for density calculations
            layer_hash         = {//"nodes"      => tmp_nodes,
								 "out_color1" => aM8_color1,
								 "out_color2" => aM8_color2,},
            window_function    = color_balance_func_M,
            merge_errors = true );

}