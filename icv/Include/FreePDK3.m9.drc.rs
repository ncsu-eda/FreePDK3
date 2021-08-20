// 3nm FreePDK(TM) M9 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM9_1 @= { @ "M9.1 : METAL9 width minimum >= 40nm";
    //Sushant
	internal1( aM9, < 0.04, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
}

rM9_2 @= { @ "M9.2 : METAL9 spacing minimum >= 40nm";
    //Sushant
	external1(aM9, < 0.04, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}

rM9_3 @= { @ "M9.3 : METAL9 maximum width in horizontal direction = 2000nm";
    //Sushant
    m9_w_2000 = internal1( aM9, < 2.0005, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
	aM9 not m9_w_2000;	
}

m9_w_120 =wide(aM9, distance >= 0.120);
m9_w_360 =wide(aM9, distance >= 0.360);	
m9_w_1080 = wide(aM9, distance >= 1.080);

m9_w_120_360 = m9_w_120 not m9_w_360;
m9_w_360_1080 = m9_w_360 not m9_w_1080;


rM9_4 @= { @ "M9.4 : Minimum spacing of METAL9 wider than 120nm and longer than 120nm >= 120nm";
    //Sushant
    m9_120 = aM9 not m9_w_120_360;
	mLAYER_9_1 = external2(m9_w_120_360,m9_120,distance < 0.120, extension = NONE,projection_length >= 0.120, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_9_2 = external1(m9_w_120_360,distance < 0.120, extension = NONE,projection_length >= 0.120, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_9_1 or mLAYER_9_2;
}

rM9_5 @= { @ "M9.5 : Minimum spacing of METAL9 wider than 360nm and longer than 360nm >= 360nm";
    //Sushant
    m9_360 = aM9 not m9_w_360_1080;
	mLAYER_9_3 = external2(m9_w_360_1080,m9_360,distance < 0.360, extension = NONE,projection_length >= 0.360, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_9_4 = external1(m9_w_360_1080,distance < 0.360, extension = NONE,projection_length >= 0.360, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_9_3 or mLAYER_9_4;	
}

rM9_6 @= { @ "M9.6 : Minimum spacing of METAL9 wider than 1080nm and longer than 1080nm >= 1080nm";
    //Sushant
	mLAYER_9_5 = external2(m9_w_1080,aM9,distance < 1.080, extension = NONE,projection_length >= 1.080, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_9_6 = external1(m9_w_1080,distance < 1.080, extension = NONE,projection_length >= 1.080, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_9_5 or mLAYER_9_6;	
}

rM9_7 @= { @ "M9.7 : Double patterning Error";
    //Sushant
	
	m9_Links =  external1_error(aM9, < 0.04, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT);   
	m9_conflict = two_color(aM9, m9_Links, aM9_color1, aM9_color2, even_loops, pre_color_layer ,output_even_loops = true, output_type=ODD_LOOP_INSIDE_RING, color_preference = BALANCED ); 
	copy(m9_conflict);
	color_output = gds_library("m9_dpt_output.gds");
	write_gds(color_output, layers = {{aM9, {1}}, {aM9_color1, {2}}, {aM9_color2, {3}}, {m9_conflict, {4}}});
}

rM9_8 @= { @ "M9.8 : Density Balancing Rule: The density of decomposed metals should be between 23 and 77%";
	//Sushant 
	density(window_layer = chip_extent(), // The polygon layer that defines the boundaries where layers are processed for density calculations
            layer_hash         = {//"nodes"      => tmp_nodes,
								 "out_color1" => aM9_color1,
								 "out_color2" => aM9_color2,},
            window_function    = color_balance_func_M,
            merge_errors = true );

}