// 3nm FreePDK(TM) M7 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM7_1 @= { @ "M7.1 : METAL7 width minimum >= 40nm";
    //Sushant
	internal1( aM7, < 0.04, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
}

rM7_2 @= { @ "M7.2 : METAL7 spacing minimum >= 40nm";
    //Sushant
	external1(aM7, < 0.04, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM7_3 @= { @ "M7.3 : METAL7 maximum width in horizontal direction = 2000nm";
    //Sushant
    m7_w_2000 = internal1( aM7, < 2.0005, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aM7 not m7_w_2000;
}

m7_w_120 =wide(aM7, distance >= 0.120);
m7_w_360 =wide(aM7, distance >= 0.360);	
m7_w_1080 = wide(aM7, distance >= 1.080);

m7_w_120_360 = m7_w_120 not m7_w_360;
m7_w_360_1080 = m7_w_360 not m7_w_1080;


rM7_4 @= { @ "M7.4 : Minimum spacing of METAL7 wider than 120nm and longer than 120nm >= 120nm";
    //Sushant
    m7_120 = aM7 not m7_w_120_360;
	mLAYER_7_1 = external2(m7_w_120_360,m7_120,distance < 0.120, extension = NONE,projection_length >= 0.120, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_7_2 = external1(m7_w_120_360,distance < 0.120, extension = NONE,projection_length >= 0.120, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_7_1 or mLAYER_7_2;
}

rM7_5 @= { @ "M7.5 : Minimum spacing of METAL7 wider than 360nm and longer than 360nm >= 360nm";
    //Sushant
    m7_360 = aM7 not m7_w_360_1080;
	mLAYER_7_3 = external2(m7_w_360_1080,m7_360,distance < 0.360, extension = NONE,projection_length >= 0.360, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_7_4 = external1(m7_w_360_1080,distance < 0.360, extension = NONE,projection_length >= 0.360, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_7_3 or mLAYER_7_4;	
}

rM7_6 @= { @ "M7.6 : Minimum spacing of METAL7 wider than 1080nm and longer than 1080nm >= 1080nm";
    //Sushant
	mLAYER_7_5 = external2(m7_w_1080,aM7,distance < 1.080, extension = NONE,projection_length >= 1.080, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_7_6 = external1(m7_w_1080,distance < 1.080, extension = NONE,projection_length >= 1.080, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_7_5 or mLAYER_7_6;	
}

rM7_7 @= { @ "M7.7 : Double patterning Error";
    //Sushant
	
	m7_Links =  external1_error(aM7, < 0.04, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	m7_conflict = two_color(aM7, m7_Links, aM7_color1, aM7_color2, even_loops, pre_color_layer ,output_even_loops = true, output_type=ODD_LOOP_INSIDE_RING, color_preference = BALANCED ); 
	copy(m7_conflict);
	color_output = gds_library("m7_dpt_output.gds");
	write_gds(color_output, layers = {{aM7, {1}}, {aM7_color1, {2}}, {aM7_color2, {3}}, {m7_conflict, {4}}});
	
	// aM7_color1 = color1_out;
	// aM7_color2 = color2_out;
}

rM7_8 @= { @ "M7.8 : Density Balancing Rule: The density of decomposed metals should be between 23 and 77%";
	//Sushant 
	density(window_layer = chip_extent(), // The polygon layer that defines the boundaries where layers are processed for density calculations
            layer_hash         = {//"nodes"      => tmp_nodes,
								 "out_color1" => aM7_color1,
								 "out_color2" => aM7_color2,},
            window_function    = color_balance_func_M,
            merge_errors = true );

}

