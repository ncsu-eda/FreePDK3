// 3nm FreePDK(TM) M1 ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

rM1_1 @= { @ "M1.1 : METAL1 width minimum >= 14nm";
    //Sushant
	internal1( aM1, < 0.014, extension = NONE, extension_look_past = POINT_TO_POINT, intersecting = {  }, projection_filter = MUTUAL_NON_ORTHOGONAL, projection_mode = ASYMMETRIC, direction = HORIZONTAL, orthogonal = BOTH );
}

rM1_2 @= { @ "M1.2 : METAL1 spacing minimum >= 14nm";
    //Sushant
	external1(aM1, < 0.014, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
}


m1_w_015 =wide(aM1, distance >= 0.015); //Extract polygons wider than 15nm
m1_w_030 =wide(aM1, distance >= 0.030); //Extract polygons wider than 30nm


m1_w_045 =wide(aM1, distance >= 0.045); //Extract polygons wider than 45nm
m1_w_135 =wide(aM1, distance >= 0.135);	//Extract polygons wider than 135nm
m1_w_405 = wide(aM1, distance >= 0.405);// Extract polygons wider than 405nm

m1_w_045_135 = m1_w_045 not m1_w_135; //Extract polygons wider than 45 but less than 135nm
m1_w_135_405 = m1_w_135 not m1_w_405; //Extract polygons wider than 135 but less than 405nm


rM1_4 @= { @ "M1.4 : Minimum spacing of METAL1 wider than 45nm and longer than 45nm >= 45nm";
    //Sushant
    m1_045 = aM1 not m1_w_045_135; //Extract polygons with width less than 45nm and greater than 135nm
	
	//find parallel polygons - one with width within 45nm and 135nm and the other with width less than 45 or greater than 135nm and check if the spacing is less than 45nm. (also check the parallel run length should be >= 45nm)
	mLAYER_1_1 = external2(m1_w_045_135,m1_045,distance < 0.045, extension = NONE,projection_length >= 0.045, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL); 
	//find parallel polygons - both with width within 45nm and 135nm and check if the spacing is less than 45nm. (also check the parallel run length should be >= 45nm)
	mLAYER_1_2 = external1(m1_w_045_135,distance < 0.045, extension = NONE,projection_length >= 0.045, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_1_1 or mLAYER_1_2;
}

rM1_5 @= { @ "M1.5 : Minimum spacing of METAL1 wider than 135nm and longer than 135nm >= 135nm";
    //Sushant
    m1_135 = aM1 not m1_w_135_405; //Extract polygons with width less than 135nm and greater than 405nm
	
	//find parallel polygons - one with width within 135nm and 405nm and the other with width less than 135 or greater than 405nm and check if the spacing is less than 135nm. (also check the parallel run length should be >= 135nm)
	mLAYER_1_3 = external2(m1_w_135_405,m1_135,distance < 0.135, extension = NONE,projection_length >= 0.135, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	//find parallel polygons - both with width within 135nm and 405nm and check if the spacing is less than 135nm. (also check the parallel run length should be >= 135nm)
	mLAYER_1_4 = external1(m1_w_135_405,distance < 0.135, extension = NONE,projection_length >= 0.135, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_1_3 or mLAYER_1_4;	
}

rM1_6 @= { @ "M1.6 : Minimum spacing of METAL1 wider than 405nm and longer than 405nm >= 405nm";
    //Sushant
	
	//find parallel polygons - one with width greater than 405nm and check if the spacing is less than 405nm. (also check the parallel run length should be >= 405nm)
	mLAYER_1_5 = external2(m1_w_405,aM1,distance < 0.405, extension = NONE,projection_length >= 0.405, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	//find parallel polygons - both with width greater than 405nm and check if the spacing is less than 405nm. (also check the parallel run length should be >= 405nm)
	mLAYER_1_6 = external1(m1_w_405,distance < 0.405, extension = NONE,projection_length >= 0.405, orientation = PARALLEL, intersecting = {}, projection = IN, orthogonal = ALL,look_thru = ALL);
	mLAYER_1_5 or mLAYER_1_6;	
}

rM1_7 @= { @ "M1.7 : Double patterning Error";
    //Sushant
	
	m1_Links = external1_error(aM1, < 0.015, extension = RADIAL, intersecting = { }, intersection_angle = < 90, look_thru = NONE, relational = { }, extension_look_past = POINT_TO_POINT); 
	m1_conflict = two_color(aM1, m1_Links, aM1_color1, aM1_color2, even_loops, pre_color_layer ,output_even_loops = true, output_type=ODD_LOOP_INSIDE_RING, color_preference = BALANCED ); 
	copy(m1_conflict);
	color_output = gds_library("m1_dpt_output.gds");
	write_gds(color_output, layers = {{aM1, {1}}, {aM1_color1, {2}}, {aM1_color2, {3}}, {m1_conflict, {4}}});	
}

rM1_8 @= { @ "M1.8 : Density Balancing Rule: The density of decomposed metals should be between 23 and 77%";
	//Sushant 
	density(window_layer = chip_extent(), // The polygon layer that defines the boundaries where layers are processed for density calculations
            layer_hash         = {//"nodes"      => tmp_nodes,
								 "out_color1" => aM1_color1,
								 "out_color2" => aM1_color2,},
            window_function    = color_balance_func_M,
            merge_errors = true );

}

