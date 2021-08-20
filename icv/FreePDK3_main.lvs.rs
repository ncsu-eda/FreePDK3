// 3nm FreePDK(TM) ICV LVS Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.


#define ICV_ENABLE_LVS_SEN_EXCLUDE_TOLERANCE
#include <icv.rh>

#ifdef VERSION_LT
#if VERSION_LT(2020, 9, 0, 0)
#error This PXL runset was generated to run with ICV version 2020.09 and newer.
#endif
#endif

#ifdef OVERRIDE_FILE
#include "override_file.rs"
#endif

layout_case_sensitive : boolean = false;
layout_preserve_case : boolean = false;
run_options_uppercase : boolean = !layout_case_sensitive && !layout_preserve_case;


//vFIN_HEIGHT : const double = 30e-9;

vFIN_WIDTH : const double = 8e-9;

//vFIN_SEPARATION : const double = 40e-9;

vWFIN_D : const double = 8e-9;

vWFIN_S : const double = 8e-9;

CONNECT_DB : connect_database = NULL_CONNECT_DATABASE;
soft_connect_layers : list of soft_connect_item_s = { };
abort_on_softchk : boolean = false;
infinity : const double = 1.0 / 0.0;

cal_text_layer_item_h : newtype hash of string to text_layer_item_s;
cal_text_layer_items_h : newtype hash of string to list of text_layer_item_s;

attach_text_net_items : cal_text_layer_items_h = {};
connect_text_net_items : cal_text_layer_item_h = {};
presort_text_net_items : list of text_layer_item_s = {};
merge_open_net_names_items : list of string = { "" };

user_functions_path : string = "FreePDK3_main.lvs.rs.user_functions"; 
vLVS_POWER_NAMES : list of string = { "VDD", "vdd", "vdd!", "VDD!" };
vLVS_GROUND_NAMES : list of string = { "VSS", "GND!", "gnd!", "vss" };

/* Begin of Equation Code-Based Functions definitions */

collect_text_net_items : published function (
    attach_items : cal_text_layer_items_h,
    connect_items : cal_text_layer_item_h,
    presort_items : list of text_layer_item_s
)

returning text_net_items : list of text_layer_item_s
{
    /* Initialize the output list. */
    text_net_items = {};

    /* Append the ATTACH-based items. */
    foreach (text_layer_items in attach_items.values()) {
        foreach (text_layer_item in text_layer_items) {
            text_net_items.push_back(text_layer_item);
        }
    }

    /* Append the CONNECT-based items which are not already present
     * because of an ATTACH.
     */
    foreach (text_layer_str in connect_items.keys()) {
        if (!attach_items.contains_key(text_layer_str)) {
            text_net_items.push_back(connect_items[text_layer_str]);
        }
    }

    /* Append the TEXT-based items. */
    foreach (text_layer_item in presort_items) {
        text_net_items.push_back(text_layer_item);
    }
}

/* End of Equation Code-Based Functions definitions */

/* Begin of Common Device Functions definitions */                                   //Needs to be determined
dev_func_MY_FET_PROPERTIES : function (void) returning void
{
    vADEJ   : double;
    vASEJ   : double;
    vL      : double;
    vLFIN_D : double;
    vLFIN_S : double;
    vNFIN   : double;
    vPDEJ   : double;
    vPSEJ   : double;
    vW      : double;

    vGATE = dev_recognition_layer();
    vS = dev_pin("SRC");
    vD = dev_pin("DRN");

    vL = ( ( dev_polygon_perim ( vGATE ) - dev_touch_length ( vGATE , vS ) - dev_touch_length ( vGATE , vD ) ) * 0.5 );
    vW = ( ( dev_touch_length ( vGATE , vS ) + dev_touch_length ( vGATE , vD ) ) * 0.5 );
    vNFIN = 1;
    vLFIN_D = ( ( dev_polygon_perim ( vD ) - ( 2 * dev_touch_length ( vGATE , vD ) ) ) * 0.5 );
    vLFIN_S = ( ( dev_polygon_perim ( vS ) - ( 2 * dev_touch_length ( vGATE , vS ) ) ) * 0.5 );
    vADEJ = ( vNFIN * vWFIN_D * vLFIN_D );
    vASEJ = ( vNFIN * vWFIN_S * vLFIN_S );
    vPDEJ = ( ( 2 * vLFIN_D * vNFIN ) + ( vWFIN_D * vNFIN ) );
    vPSEJ = ( ( 2 * vLFIN_S * vNFIN ) + ( vWFIN_S * vNFIN ) );
    if ( dblcmp( vLFIN_D, vLFIN_S, DBL_GT ) ) {
        vLFIN_D = ( vLFIN_D * 0.5 );
        vADEJ = ( vNFIN * vWFIN_D * vLFIN_D );
        vPDEJ = ( 2 * vLFIN_D * vNFIN );
    }
    if ( dblcmp( vLFIN_S, vLFIN_D, DBL_GT ) ) {
        vLFIN_S = ( vLFIN_S * 0.5 );
        vASEJ = ( vNFIN * vWFIN_S * vLFIN_S );
        vPSEJ = ( 2 * vLFIN_S * vNFIN );
    }


    dev_save_double_properties({
        { "W", vW },
        { "L", vL },
        { "nfin", vNFIN },
        { "ADEJ", vADEJ },
        { "ASEJ", vASEJ },
        { "PDEJ", vPDEJ },
        { "PSEJ", vPSEJ }
    });
}
/* End of Common Device Functions definitions */

library(
    cell         = "cell",
    format       = GDSII,
    library_name = "inlib"
);

schematic_db = schematic(
    schematic_file = {{"sch", SPICE}}
);

run_options(
    lvs_netlist_flow = SPICE,
    lvs_user_unit    = METER,
    uppercase        = run_options_uppercase
);

compatibility_options(
    drc = { { orphan_edge_membership = NONE } }
);

error_options(
    error_limit_per_check   = 1000,
    report_empty_violations = true
);

gds_options(
    duplicate_cell = DROP
);

resolution_options(
    drc_angle_precision  = 0.0,
    drc_length_precision = 0.0,
    internal_resolution  = 0.0005,
    spacing_tolerance    = 0.0
);

layout_drawn_options(
    self_intersect_action = FILL
);

layout_grid_options(
    check_45 = {},
    check_90 = {}
);

text_options(
    allow_all_numeric             = true,
    colon_text                    = REGULAR_TEXT,
    layout_ground                 = vLVS_GROUND_NAMES,
    layout_power                  = vLVS_POWER_NAMES,
    replace_text_characters_regex = { { { { search_string = "[\\s\\*\"={},]", replace_string = "_" } } } },
    semicolon_text                = REGULAR_TEXT
);

net_options(
    schematic_ground = vLVS_GROUND_NAMES,
    schematic_power  = vLVS_POWER_NAMES
);

lvs_options(
    spice_multiplier_names = { "M" }
);

////////////////////////////////////////////////////////////////////////////////////
//
//    3nm Free PDK  Rules
//
////////////////////////////////////////////////////////////////////////////////////

//assigning layer numbers for wire, text, via
aBPR		= assign({ { 0 } });
tBPR		= assign_text({ { 0 } });
aVBPR		= assign({ { 1 } });
aNW			= assign({ { 2 } });
aACT		= assign({ { 3 } });
aGATE		= assign({ { 4 } });
aGCUT      	= assign({ { 5 } });
aDUMMY		= assign({ { 6 } });
aNIM       	= assign({ { 7 } });
aPIM       	= assign({ { 8 } });
aM0A       	= assign({ { 9 } });
tM0A		= assign_text({ { 9 } });
aV0A       	= assign({ { 10 } });
aGCON      	= assign({ { 11 } });
aM0B       	= assign({ { 12 } });
tM0B		= assign_text({ { 12 } });
aV0B       	= assign({ { 13 } });
aM1        	= assign({ { 14 } });
tM1		    = assign_text({ { 14 } });
aV1			= assign({ { 15 } });
aM2			= assign({ { 16 } });
tM2		    = assign_text({ { 16 } });
aV2         = assign({ { 17 } });
aM3         = assign({ { 18 } }); 
tM3         = assign_text({ { 18 } }); 
aV3         = assign({ { 19 } }); 
aM4         = assign({ { 20 } }); 
tM4         = assign_text({ { 20 } }); 
aV4         = assign({ { 21 } }); 
aM5         = assign({ { 22 } }); 
tM5         = assign_text({ { 22 } }); 
aV5         = assign({ { 23 } }); 
aM6         = assign({ { 24 } }); 
tM6         = assign_text({ { 24 } }); 
aV6         = assign({ { 25 } }); 
aM7         = assign({ { 26 } }); 
tM7         = assign_text({ { 26 } }); 
aV7         = assign({ { 27 } }); 
aM8         = assign({ { 28 } }); 
tM8         = assign_text({ { 28 } }); 
aV8         = assign({ { 29 } }); 
aM9         = assign({ { 30 } }); 
tM9         = assign_text({ { 30 } }); 
aV9         = assign({ { 31 } }); 
aM10        = assign({ { 32 } }); 
tM10        = assign_text({ { 32 } }); 
aV10        = assign({ { 33 } }); 
aM11        = assign({ { 34 } }); 
tM11        = assign_text({ { 34 } }); 
aV11        = assign({ { 35 } }); 
aM12        = assign({ { 36 } }); 
tM12        = assign_text({ { 36 } }); 
aV12        = assign({ { 37 } }); 
aM13        = assign({ { 38 } }); 
tM13        = assign_text({ { 38 } }); 
aVRDL       = assign({ { 39 } }); 
aRDL       = assign({ { 40 } }); 
tRDL       = assign_text({ { 40 } }); 



//linking text to metal in that layer
//connect_text_net_items["<text layer name>"] = { <assigned layer>, <text layer> }; //used to attach text layer to assigned layer
connect_text_net_items["tBPR"] = { aBPR, tBPR };
connect_text_net_items["tM0A"] = { aM0A, tM0A };
connect_text_net_items["tM0B"] = { aM0B, tM0B };
connect_text_net_items["tM1"] = { aM1, tM1 };
connect_text_net_items["tM2"] = { aM2, tM2 };
connect_text_net_items["tM3"] = { aM3, tM3 };
connect_text_net_items["tM4"] = { aM4, tM4 };
connect_text_net_items["tM5"] = { aM5, tM5 };
connect_text_net_items["tM6"] = { aM6, tM6 };
connect_text_net_items["tM7"] = { aM7, tM7 };
connect_text_net_items["tM8"] = { aM8, tM8 };
connect_text_net_items["tM9"] = { aM9, tM9 };
connect_text_net_items["tM10"] = { aM10, tM10 };
connect_text_net_items["tM11"] = { aM11, tM11 };
connect_text_net_items["tM12"] = { aM12, tM12 };
connect_text_net_items["tM13"] = { aM13, tM13 };
connect_text_net_items["tRDL"] = { aRDL, tRDL };



//connecting layers with vias
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aBPR, aM0A }, aVBPR, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM0A, aM0B }, aV0A, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM0B, aM1 }, aV0B, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM1, aM2 }, aV1, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM2, aM3 }, aV2, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM3, aM4 }, aV3, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM4, aM5 }, aV4, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM5, aM6 }, aV5, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM6, aM7 }, aV6, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM7, aM8 }, aV7, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM8, aM9 }, aV8, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM9, aM10 }, aV9, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM10, aM11 }, aV10, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM11, aM12 }, aV11, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM12, aM13 }, aV12, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM13, aRDL }, aVRDL, NONE, SHIELDED_OVERLAP }} );



// gPOLY_temp = ( ( aGATE ) ) not aGCUT;
gPOLY = (( ( aGATE ) ) not aGCUT) not aDUMMY; // excluding poly area overlapped by DUMMY layer

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gPOLY, aM0B }, aGCON, NONE, SHIELDED_OVERLAP }} ); 

text_net_items = collect_text_net_items( attach_text_net_items, connect_text_net_items, presort_text_net_items );
if (!text_net_items.empty()) {
    CONNECT_DB = text_net( CONNECT_DB, text_layer_items = text_net_items, use_text = TOP, attach_text = ALL, opens = MERGE_CONNECTED_AND_TOP, merge_open_net_names = merge_open_net_names_items, rename_open_nets = KEEP_ONE );
}

//gPOLY = ( ( aGATE ) ) not aGCUT;

//soft_connect_layers.push_back( { aDUMMY, { gPOLY } } ); 

gBULK = cell_extent( cell_list = { "*" }, exclude_text_layers = false, extent_layers = USED_ASSIGN_LAYERS );

gPWELL = gBULK not aNW;

gN_ACTIVE_AREA = gPWELL and aACT;

gN_FIN_AREA = gN_ACTIVE_AREA and aNIM;

gNGATE1 = gN_FIN_AREA and gPOLY;

// gNFINS_temp = gN_FIN_AREA not gNGATE1;

gNFINS = (gN_FIN_AREA not gNGATE1) not aDUMMY; // excluding fin area overlapped by DUMMY layer

soft_connect_layers.push_back( { aM0A, { gNFINS } } );

gNWELL = gBULK and aNW;

gP_ACTIVE_AREA = gNWELL and aACT;//changed from M0A to ACT

gP_FIN_AREA = gP_ACTIVE_AREA and aPIM;

gPGATE1 = gP_FIN_AREA and gPOLY;

//gPFINS_temp = gP_FIN_AREA not gPGATE1;

gPFINS = (gP_FIN_AREA not gPGATE1) not aDUMMY; // excluding fin area overlapped by DUMMY layer

soft_connect_layers.push_back( { aM0A, { gPFINS } } );

gVPW1 = aPIM and aACT;

gVPW = gVPW1 and gPWELL;

soft_connect_layers.push_back( { aM0A, { gPWELL }, gVPW } );

gVNW1 = aNIM and aM0A;

gVNW = gVNW1 and gNWELL;

soft_connect_layers.push_back( { aM0A, { gNWELL }, gVNW } );

if (!soft_connect_layers.empty()) {
   CONNECT_DB = soft_connect( CONNECT_DB, soft_connect_layers, conflict_resolution = TRAPEZOID_COUNT );
}

//SOFTCHK_gPOLY_LOWER @= { @ "SOFTCHK_gPOLY_LOWER";
    //soft_connect_check( CONNECT_DB, gPOLY, LOWER );
//}
//if (abort_on_softchk && !violation_empty(SOFTCHK_gPOLY_LOWER)) {
    //error("SOFTCHK_gPOLY_LOWER produced violations.  Exiting.");
//}

CONNECT_DB = create_ports(CONNECT_DB, { {aBPR, aBPR}, {aM0A, aM0A}, {aM0B, aM0B}, {aM1, aM1}, {aM2, aM2}, {aM3, aM3}, {aM4, aM4}, {aM5, aM5}, {aM6, aM6}, {aM7, aM7}, {aM8, aM8}, {aM9, aM9}, {aM10, aM10}, {aM11, aM11}, {aM12, aM12}, {aM13, aM13}, {aRDL, aRDL} });

dev_matrix : device_matrix = init_device_matrix(CONNECT_DB);

nmos( bulk_relationship = INTERACT, device_name = "nmos", drain = gNFINS, gate = gPOLY, matrix = dev_matrix, properties = { { "W" }, { "L" }, { "nfin" }, { "ADEJ" }, { "ASEJ" }, { "PDEJ" }, { "PSEJ" } }, property_function = dev_func_MY_FET_PROPERTIES, recognition_layer = ( gNGATE1 interacting gPWELL ), simulation_model_name = "nmos", source = gNFINS );

pmos( bulk_relationship = INTERACT, device_name = "pmos", drain = gPFINS, gate = gPOLY, matrix = dev_matrix, properties = { { "W" }, { "L" }, { "nfin" }, { "ADEJ" }, { "ASEJ" }, { "PDEJ" }, { "PSEJ" } }, property_function = dev_func_MY_FET_PROPERTIES, recognition_layer = ( gPGATE1 interacting gNWELL ), simulation_model_name = "pmos", source = gPFINS );

//removed from NMOS for ignoring bulk terminal - optional_pins = { { device_layer = gPWELL,  pin_compared = false}  },

//removed from PMOS for ignoring bulk terminal - optional_pins = { { device_layer = gNWELL,  pin_compared = false } },

device_db = extract_devices(dev_matrix);

layout_netlist_db = netlist( device_db = device_db, include_empty_cells = NONE, precision = 6 );
compare_settings = init_compare_matrix();
filter_off(state = compare_settings, device_type = CAPACITOR);
filter_off(state = compare_settings, device_type = RESISTOR);
filter_off(state = compare_settings, device_type = NMOS);
filter_off(state = compare_settings, device_type = PMOS);
filter_off(state = compare_settings, device_type = NP);
filter_off(state = compare_settings, device_type = PN);
filter_off(state = compare_settings, device_type = NPN);
filter_off(state = compare_settings, device_type = PNP);
merge_parallel(state = compare_settings, device_type = NPN, property_functions = { { "default_par_wl_props"} });
merge_parallel(state = compare_settings, device_type = PNP, property_functions = { { "default_par_wl_props"} });
merge_parallel(state = compare_settings, device_type = CAPACITOR, property_functions = { { "default_par_cap_props"} });
merge_parallel(state = compare_settings, device_type = NP);
merge_parallel(state = compare_settings, device_type = PN);
merge_parallel(state = compare_settings, device_type = NMOS, property_functions = { { "default_par_mos_props"} });
merge_parallel(state = compare_settings, device_type = PMOS, property_functions = { { "default_par_mos_props"} });
merge_parallel(state = compare_settings, device_type = RESISTOR, property_functions = { { "default_par_wl_props"} });
merge_series(state = compare_settings, device_type = CAPACITOR);
merge_series_off(state = compare_settings, device_type = NMOS);
merge_series_off(state = compare_settings, device_type = PMOS);
merge_series(state = compare_settings, device_type = RESISTOR, property_functions = { { "default_ser_wl_props"} });

recognize_gate(state = compare_settings, type = ALL);
short_equivalent_nodes(state = compare_settings, device_type = NMOS, exclude_tolerances = { { "W", [ -infinity, infinity ], RELATIVE, RATIO } });
short_equivalent_nodes(state = compare_settings, device_type = PMOS, exclude_tolerances = { { "W", [ -infinity, infinity ], RELATIVE, RATIO } });

compare(
   user_functions_file = search_include_path("FreePDK3_main.lvs.rs.user_functions"),
   state = compare_settings,
   schematic = schematic_db,
   layout = layout_netlist_db
);


//add pex functions here - vpasuma

pex_matrix = init_pex_layer_matrix(device_db);
pex_conducting_layer_map(pex_matrix, gNWELL, "SUBSTRATE", tagname="SUBSTRATENWELL");
pex_conducting_layer_map(pex_matrix, gPWELL, "SUBSTRATE", tagname="SUBSTRATEPWELL");
pex_conducting_layer_map(pex_matrix, gVNW, "SUBSTRATE", tagname="SUBSTRATEVNW");
pex_conducting_layer_map(pex_matrix, gVPW, "SUBSTRATE", tagname="SUBSTRATEVPW");
pex_conducting_layer_map(pex_matrix, gNFINS, "ACT", tagname="ACTNFINS");
pex_conducting_layer_map(pex_matrix, gPFINS, "ACT", tagname="ACTPFINS");
pex_conducting_layer_map(pex_matrix, gPOLY, "GATE", tagname="GATEPOLY");
pex_conducting_layer_map(pex_matrix, aM0A, "M0A", tagname="M0A");
pex_conducting_layer_map(pex_matrix, aM0B, "M0B", tagname="M0B");
pex_conducting_layer_map(pex_matrix, aM1, "M1", tagname="M1");
pex_conducting_layer_map(pex_matrix, aM2, "M2", tagname="M2");
pex_conducting_layer_map(pex_matrix, aM3, "M3", tagname="M3");
pex_conducting_layer_map(pex_matrix, aM4, "M4", tagname="M4");
pex_conducting_layer_map(pex_matrix, aM5, "M5", tagname="M5");
pex_conducting_layer_map(pex_matrix, aM6, "M6", tagname="M6");
pex_conducting_layer_map(pex_matrix, aM7, "M7", tagname="M7");
pex_conducting_layer_map(pex_matrix, aM8, "M8", tagname="M8");
pex_conducting_layer_map(pex_matrix, aM9, "M9", tagname="M9");
pex_conducting_layer_map(pex_matrix, aM10, "M10", tagname="M10");
pex_conducting_layer_map(pex_matrix, aM11, "M11", tagname="M11");
pex_conducting_layer_map(pex_matrix, aM12, "M12", tagname="M12");
pex_conducting_layer_map(pex_matrix, aM13, "M13", tagname="M13");
pex_conducting_layer_map(pex_matrix, aRDL, "RDL", tagname="RDL");
pex_conducting_layer_map(pex_matrix, aBPR, "BPR", tagname="BPR"); //Modified by Sushant

pex_via_layer_map(pex_matrix, aVBPR, "VBPR", tagname="VBPR"); //Modified by Sushant
pex_via_layer_map(pex_matrix, aGCON, "GCON", tagname="GCON");
pex_via_layer_map(pex_matrix, aV0A, "V0A", tagname="V0A");
pex_via_layer_map(pex_matrix, aV0B, "V0B", tagname="V0B");
pex_via_layer_map(pex_matrix, aV1, "V1", tagname="V1");
pex_via_layer_map(pex_matrix, aV2, "V2", tagname="V2");
pex_via_layer_map(pex_matrix, aV3, "V3", tagname="V3");
pex_via_layer_map(pex_matrix, aV4, "V4", tagname="V4");
pex_via_layer_map(pex_matrix, aV5, "V5", tagname="V5");
pex_via_layer_map(pex_matrix, aV6, "V6", tagname="V6");
pex_via_layer_map(pex_matrix, aV7, "V7", tagname="V7");
pex_via_layer_map(pex_matrix, aV8, "V8", tagname="V8");
pex_via_layer_map(pex_matrix, aV9, "V9", tagname="V9");
pex_via_layer_map(pex_matrix, aV10, "V10", tagname="V10");
pex_via_layer_map(pex_matrix, aV11, "V11", tagname="V11");
pex_via_layer_map(pex_matrix, aV12, "V12", tagname="V12");
pex_via_layer_map(pex_matrix, aVRDL, "VRDL", tagname="VRDL");


pex_layout_library_handle = gds_library("pex.gds");
pex_layout_library_layer_map_handle = pex_library_layer_map_file("pex.gds.layer.map");
pex_spice_netlist_file_handle = spice_netlist_file("pex.sp");
pex_cell_extents_file_handle = pex_cell_extents_file("pex.cell.extents");
pex_cell_port_file_handle = pex_cell_port_file("pex.cell.port");
pex_process_handle = pex_process_map_file("pex.process.map");
pex_report_handle = pex_runset_report_file("pex_runset_report");


pex_generate_database(
    pex_matrix = pex_matrix,
    pex_library = pex_layout_library_handle,
    pex_library_layer_map = pex_layout_library_layer_map_handle,
    pex_netlist_file = pex_spice_netlist_file_handle,
    pex_cell_extents_file = pex_cell_extents_file_handle,
    pex_cell_port_file = pex_cell_port_file_handle,
    pex_process_map_file = pex_process_handle,
    pex_runset_report_file = pex_report_handle
);

