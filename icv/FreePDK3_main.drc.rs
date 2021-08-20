// 3nm FreePDK(TM) ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.



#define SELECTABLE_VIOLATION_NAMES
#define ICV_ENABLE_WIDE_ANGLED
#include <icv.rh>
#include "string.rh"

error_options( 
error_limit_per_check   = 
#if d_ENV_ERROR_LIMITATION == dx_UNLIMITED
ERROR_LIMIT_UNLIMITED
#else
1000
#endif
,report_empty_violations = true
);

//FreePDK3 Variables
	ANTENNA_RATIO = 50;
//FreePDK3 Variables

//FreePDK3 Functions
	out_color1 : string = "color1_out";
	out_color2 : string = "color1_out";

	color_balance_func_M : function (void) returning void
	{
		_den_con_1 = < 0.230;
		_den_con_2 = > 0.770;
		areaL1 = den_polygon_area("out_color1", clip = false);   
		areaL2 = den_polygon_area("out_color2", clip = false);
		ratio = areaL1 / (areaL1 + areaL2);
		if ( (double_constraint_overlap(_den_con_1, ratio) ||  double_constraint_overlap(_den_con_2, ratio)) && !isinf(ratio) && !isnan(ratio) && dblne(ratio, 100.00) && dblne(ratio, 0.0)) {
				den_save_window(
								error_names = { "ratio", "areaL1", "areaL2" },
								values      = { ratio, areaL1, areaL2 }
				);
		}
	}

//FreePDK3 Functions

// Assign layer section
aBPR                   = assign({ {0, 0} });
aVBPR                  = assign({ {1, 0} });
aNW                    = assign({ {2, 0} });
aACT                   = assign({ {3, 0} });
aGATE                  = assign({ {4, 0} });
aGCUT                  = assign({ {5, 0} });
aDUMMY                 = assign({ {6, 0} });
aNIM                   = assign({ {7, 0} });
aPIM                   = assign({ {8, 0} });
aM0A                   = assign({ {9, 0} });
aV0A                   = assign({ {10, 0} });
aGCON                  = assign({ {11, 0} });
aM0B                   = assign({ {12, 0} });
aV0B                   = assign({ {13, 0} });
aM1                    = assign({ {14, 0} });
aM1_color1             = assign({ {14, 11}});
aM1_color2             = assign({ {14, 12}});
aV1                    = assign({ {15, 0} });
aM2                    = assign({ {16, 0} });
aM2_color1             = assign({ {16, 11}});
aM2_color2             = assign({ {16, 12}});
aV2                    = assign({ {17, 0} });
aM3                    = assign({ {18, 0} });
aM3_color1             = assign({ {18, 11}});
aM3_color2             = assign({ {18, 12}});
aV3                    = assign({ {19, 0} });
aM4                    = assign({ {20, 0} });
aV4                    = assign({ {21, 0} });
aM5                    = assign({ {22, 0} });
aV5                    = assign({ {23, 0} });
aM6                    = assign({ {24, 0} });
aV6                    = assign({ {25, 0} });
aM7                    = assign({ {26, 0} });
aM7_color1             = assign({ {26, 11}});
aM7_color2             = assign({ {26, 12}});
aV7                    = assign({ {27, 0} });
aM8                    = assign({ {28, 0} });
aM8_color1             = assign({ {28, 11}});
aM8_color2             = assign({ {28, 12}});
aV8                    = assign({ {29, 0} });
aM9                    = assign({ {30, 0} });
aM9_color1             = assign({ {30, 11}});
aM9_color2             = assign({ {30, 12}});
aV9                    = assign({ {31, 0} });
aM10                   = assign({ {32, 0} });
aV10                   = assign({ {33, 0} });
aM11                   = assign({ {34, 0} });
aV11                   = assign({ {35, 0} });
aM12                   = assign({ {36, 0} });
aV12                   = assign({ {37, 0} });
aM13                   = assign({ {38, 0} });
aVRDL                  = assign({ {39, 0} });
aRDL                   = assign({ {40, 0} });



//connection database


//cdb_v0a = connect(connect_items = {{{aV0A, aM0A, aM0B}}});
//cdb_gcon = connect(connect_items = {{{aGCON, aGATE}}});
//cdb_gate = connect (connect_items = {{{aGATE, aACT}}});

CONNECT_DB : connect_database = NULL_CONNECT_DATABASE;

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

gPOLY = ( ( aGATE ) ) not aGCUT;
gCHANNEL = gPOLY and aACT;
gN_GATE = gCHANNEL and aNIM; 
gP_GATE = gCHANNEL and aPIM;

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gPOLY, aM0B }, aGCON, NONE, SHIELDED_OVERLAP }} ); 

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gPOLY, aACT }}} ); 

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aGATE, aGCON }}} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gN_GATE, aGCON }}} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gP_GATE, aGCON }}} );

gP_GATE_or_gN_GATE = gP_GATE or gN_GATE;

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gP_GATE_or_gN_GATE , aGCON }}} );



#include "Include/FreePDK3.bpr.drc.rs"
#include "Include/FreePDK3.nimpim.drc.rs" 
#include "Include/FreePDK3.act.drc.rs"
#include "Include/FreePDK3.gcut.drc.rs"
#include "Include/FreePDK3.nw.drc.rs"
#include "Include/FreePDK3.vbpr.drc.rs"
#include "Include/FreePDK3.gcon.drc.rs"
#include "Include/FreePDK3.v0a.drc.rs" 
#include "Include/FreePDK3.v0b.drc.rs"
#include "Include/FreePDK3.m0a.drc.rs"
#include "Include/FreePDK3.m0b.drc.rs"
#include "Include/FreePDK3.gate.drc.rs"
#include "Include/FreePDK3.dummy.drc.rs"
#include "Include/FreePDK3.m1.drc.rs"
#include "Include/FreePDK3.v1.drc.rs"
#include "Include/FreePDK3.m2.drc.rs"
#include "Include/FreePDK3.v2.drc.rs"
#include "Include/FreePDK3.m3.drc.rs"
#include "Include/FreePDK3.v3.drc.rs"
#include "Include/FreePDK3.m4.drc.rs"
#include "Include/FreePDK3.v4.drc.rs"
#include "Include/FreePDK3.m5.drc.rs"
#include "Include/FreePDK3.v5.drc.rs"
#include "Include/FreePDK3.m6.drc.rs"
#include "Include/FreePDK3.v6.drc.rs"
#include "Include/FreePDK3.m7.drc.rs"
#include "Include/FreePDK3.v7.drc.rs"
#include "Include/FreePDK3.m8.drc.rs"
#include "Include/FreePDK3.v8.drc.rs"
#include "Include/FreePDK3.m9.drc.rs"
#include "Include/FreePDK3.v9.drc.rs"
#include "Include/FreePDK3.m10.drc.rs"
#include "Include/FreePDK3.v10.drc.rs"
#include "Include/FreePDK3.m11.drc.rs"
#include "Include/FreePDK3.v11.drc.rs"
#include "Include/FreePDK3.m12.drc.rs"
#include "Include/FreePDK3.v12.drc.rs"
#include "Include/FreePDK3.m13.drc.rs"
#include "Include/FreePDK3.vrdl.drc.rs"
#include "Include/FreePDK3.rdl.drc.rs"
#include "Include/FreePDK3.antenna.drc.rs"
