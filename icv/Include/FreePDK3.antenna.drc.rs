// 3nm FreePDK(TM) ANTENNA ICV Design Rules
//
// Copyright (c) 2021, North Carolina State University
// All Rights Reserved.
//
// Please see the file LICENSE included with this distribution for license.
// You may not use these files except in compliance with the License.

nar_funcs_e : newtype enum of {
    UNKNOWN,
    NAR_SAVE_NET_10
};
nar_void_func_s : newtype struct of {
    func : function (void) returning void;
};

nar_void_func_h : newtype hash of nar_funcs_e to nar_void_func_s;

nar_constrained_area_s : newtype struct of {
    area    : constraint of double = > 0;
    layer1  : string               = "";
    factor1 : double               = 1.0;
    layer2  : string               = "";
    factor2 : double               = 1.0;
};

nar_constrained_areas_h : newtype hash of string to nar_constrained_area_s;

net_area_ratio : published function (
    cdb_in            : connect_database,
    con               : constraint of double,
    layer_groups      : layer_groups_h,
    denominator_start : integer,
    denominator_end   : integer,
    constrained_areas : nar_constrained_areas_h = { },
    func_enum         : nar_funcs_e             = UNKNOWN,
    expr_is_default   : boolean                 = false
)
    returning void
{
    nar_funcs   : nar_void_func_h = { };
    nar_func    : function (void) returning void;
    con2any     : list of polygon_layer = { };
    not_con2any : list of polygon_layer = { };
    n_layer_0   : polygon_layer;

    /* Declare and register equation code function. */
    nar_save_net_10 : function (void) returning void
    {
        areaL1 = ns_net_area("layer1");
        areaL2 = ns_net_area("layer2");
        areaL3 = ns_net_area("layer3");
        areaL4 = ns_net_area("layer4");
        areaL5 = ns_net_area("layer5");
        areaL6 = ns_net_area("layer6");
        areaL7 = ns_net_area("layer7");
        areaL8 = ns_net_area("layer8");
        areaL9 = ns_net_area("layer9");
        areaL10 = ns_net_area("layer10");

        con_eq_zero : boolean = ((con.category() == CONSTRAINT_EQ) && dbleq(con.lo(), 0));
        if (( ( areaL9 + areaL10 ) > 0) || con_eq_zero) {
            ratio = ( ( areaL9 + areaL10 ) > 0) ? (( areaL1 + areaL2 + areaL3 + areaL4 + areaL5 + areaL6 + areaL7 + areaL8 ) / ( areaL9 + areaL10 )) : 0;

            if (dblccon(con, ratio)) {
                ns_save_net({"ratio"}, {ratio});
            }
        }
    }
    nar_funcs[NAR_SAVE_NET_10] = { nar_save_net_10 };

    /* Lookup the net_function to use in this num/den instance. */
    nar_func = nar_funcs[func_enum].func;

    n_layer_0 = layer_groups["layer1"][0];

    if (expr_is_default && (layer_groups.size() > 1)) {
        d_layers : list of polygon_layer = { };

        for (i = denominator_start to denominator_end) {
            d_layers.push_back(layer_groups["layer" + i][0]);
        }

        if ((con.category() == CONSTRAINT_EQ) && !((con.lo() > 0.0) || (con.lo() < 0.0))) {
            not_con2any = d_layers;
        }
        else {
            con2any = d_layers;
        }
    }

    net_select(
        connect_sequence     = cdb_in,
        net_function         = nar_func,
        layer_groups         = layer_groups,
        connected_to_all     = { n_layer_0 },
        connected_to_any     = con2any,
        not_connected_to_any = not_con2any,
        output_from_layers   = { n_layer_0 },
        error_net_output     = ALL
    );
}

rANTENNA_1 @= { @ "ANT.1 : ANTENNA Ratio of Maximum Allowed GATE Area to transistor Gate Area is 50 :1";
   //Sushant 
   net_area_ratio( CONNECT_DB, > ANTENNA_RATIO, { "layer1" => gPOLY, "layer2" => aGCON, "layer3" => aM0A, "layer4" => aM1, "layer5" => aM2, "layer6" => aM3, "layer7" => aM4, "layer8" => aM5, "layer9" => gN_GATE, "layer10" => gP_GATE, "layer11" => gP_GATE_or_gN_GATE }, 9, 10, {  }, NAR_SAVE_NET_10, true );
}