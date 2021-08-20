# 3nm FreePDK(TM) Custom Compiler Schematic Callbacks
#
# Copyright (c) 2021, North Carolina State University
# All Rights Reserved.
#
# Please see the file LICENSE included with this distribution for license.
# You may not use these files except in compliance with the License.
#

proc ncsuNFinCallBack {} {
    return [ncsuNFinCallBackDynamicImplementation]
}
proc ncsuNFinCallBackDynamicImplementation {} {
            global nfin s
            set cdfgData [iPDK_getCurrentInst]
            set nfin ""
                set nfin [expr {round([readFloat [iPDK_getParamValue "m" $cdfgData]])}]
                if {[expr {($nfin < 2)}]} {
                    set nfin 2
                    puts "[eval format \"WARNING: Minimum number of fins is 2\"]"
                }; # if
                # Set Fins
                iPDK_setParamValue "m" $nfin $cdfgData 0
                # Adjust Areas
                iPDK_setParamValue "as" [ expr { ($nfin * 304e-18) } ] $cdfgData 0
                iPDK_setParamValue "ad" [ expr { ($nfin * 304e-18) } ] $cdfgData 0
                # Adjust Peripheries
                iPDK_setParamValue "ps" [ expr { ($nfin * 84e-9) } ] $cdfgData 0
                iPDK_setParamValue "pd" [ expr { ($nfin * 84e-9) } ] $cdfgData 0
                return
}
#  ncsuWidthCallBack()                                       
#  sets: [w areas peripheries]                               
proc ncsuWidthCallBack {} {
    return [ncsuWidthCallBackDynamicImplementation]
}
proc ncsuWidthCallBackDynamicImplementation {} {
            global grid max_width min_width ncsuGlobalData s width
            set cdfgData [iPDK_getCurrentInst]
            set grid ""
            set width ""
            set max_width ""
            set min_width ""
                # Set Grid, Min & Max W values from ncsuGlobalData dict
                set grid [dict get $ncsuGlobalData GridRes]
                set max_width [expr ([expr {round([ expr { ([readFloat [dict get $ncsuGlobalData MaxW]] / $grid) } ])}] * $grid)]
                set min_width [expr ([expr {round([ expr { ([readFloat [dict get $ncsuGlobalData MinW]] / $grid) } ])}] * $grid)]
                # Ensure width is on grid
                set width [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "w" $cdfgData]] / $grid) } ])}] * $grid)]
                # If width is less than minimum, set to Min
                if {[expr {($width < $min_width)}]} {
                    set width $min_width
                    puts "[eval format \"WARNING: Minimum Width is %g M\n\" $min_width]"
                }; # if
                # If width is greater than maximum, set to Max
                if {[expr {($width > $max_width)}]} {
                    set width $max_width
                    puts "[eval format \"WARNING: Maximum Width is %g M\n\" $max_width]"
                }; # if
                # Set Width
                iPDK_setParamValue "w" $width $cdfgData 0
                # Adjust Areas
                ncsuAreaSourceCalc 
                ncsuAreaDrainCalc 
                # Adjust Peripheries
                ncsuPeriphSourceCalc 
                ncsuPeriphDrainCalc
                return
}
#  ncsuLengthCallBack()                                      
#  sets: [l]                                                 
proc ncsuLengthCallBack {} {
    return [ncsuLengthCallBackDynamicImplementation]
}
proc ncsuLengthCallBackDynamicImplementation {} {
            global grid length max_length min_length ncsuGlobalData s
            set cdfgData [iPDK_getCurrentInst]
            set grid ""
            set length ""
            set max_length ""
            set min_length ""
                # Set Grid, Min & Max L values from ncsuGlobalData dict
                set grid [dict get $ncsuGlobalData GridRes]
                set max_length [expr ([expr {round([ expr { ([readFloat [dict get $ncsuGlobalData MaxL]] / $grid) } ])}] * $grid)]
                set min_length [expr ([expr {round([ expr { ([readFloat [dict get $ncsuGlobalData MinL]] / $grid) } ])}] * $grid)]
                # Ensure length is on grid
                set length [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "l" $cdfgData]] / $grid) } ])}] * $grid)]
                # If length is less than minimum, set to Min
                if {[expr {($length < $min_length)}]} {
                    set length $min_length
                    puts "[eval format \"WARNING: Minimum Length is %g M\n\" $min_length]"
                }; # if
                # If length is greater than maximum, set to Max
                if {[expr {($length > $max_length)}]} {
                    set length $max_length
                    puts "[eval format \"WARNING: Maximum Length is %g M\n\" $max_length]"
                }; # if
                # Set Length
                iPDK_setParamValue "l" $length $cdfgData 0
                return
}
#  ncsuSourceDiffLengthCallBack()                            
#  sets: [areas peripheries]                                 
proc ncsuSourceDiffLengthCallBack {} {
    return [ncsuSourceDiffLengthCallBackDynamicImplementation]
}
proc ncsuSourceDiffLengthCallBackDynamicImplementation {} {
            global grid max_source_length min_source_length ncsuGlobalData s source_length
            set cdfgData [iPDK_getCurrentInst]
            set grid ""
            set source_length ""
            set max_source_length ""
            set min_source_length ""
                # Set Grid, Min & Max W values from ncsuGlobalData dict
                set grid [dict get $ncsuGlobalData GridRes]
                set max_source_length [expr ([expr {round([ expr { ([readFloat [dict get $ncsuGlobalData MaxSourceLength]] / $grid) } ])}] * $grid)]
                set min_source_length [expr ([expr {round([ expr { ([readFloat [dict get $ncsuGlobalData MinSourceLength]] / $grid) } ])}] * $grid)]
                # Ensure source diffusion length is on grid
                set source_length [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "ls" $cdfgData]] / $grid) } ])}] * $grid)]
                # If source difussion length is less than minimum, set to Min
                if {[expr {($source_length < $min_source_length)}]} {
                    set source_length $min_source_length
                    puts "[eval format \"WARNING: Minimum Source Diffusion Length is %g M\n\" $min_source_length]"
                }; # if
                # If source diffusion length is greater than maximum, set to Max
                if {[expr {($source_length > $max_source_length)}]} {
                    set source_length $max_source_length
                    puts "[eval format \"WARNING: Maximum Source Diffusion Length is %g M\n\" $max_source_length]"
                }; # if
                # Set Source Length
                iPDK_setParamValue "ls" $source_length $cdfgData 0
                # Recalculate Area & Periphery
                ncsuAreaSourceCalc 
                ncsuPeriphSourceCalc
                return
}
#  ncsuDrainDiffLengthCallBack()                             
#  sets: [areas peripheries]                                 
proc ncsuDrainDiffLengthCallBack {} {
    return [ncsuDrainDiffLengthCallBackDynamicImplementation]
}
proc ncsuDrainDiffLengthCallBackDynamicImplementation {} {
            global drain_length grid max_drain_length min_drain_length ncsuGlobalData s
            set cdfgData [iPDK_getCurrentInst]
            set grid ""
            set drain_length ""
            set max_drain_length ""
            set min_drain_length ""
                # Set Grid, Min & Max W values from ncsuGlobalData dict
                set grid [dict get $ncsuGlobalData GridRes]
                set max_drain_length [expr ([expr {round([ expr { ([readFloat [dict get $ncsuGlobalData MaxDrainLength]] / $grid) } ])}] * $grid)]
                set min_drain_length [expr ([expr {round([ expr { ([readFloat [dict get $ncsuGlobalData MinDrainLength]] / $grid) } ])}] * $grid)]
                # Ensure drain diffusion length is on grid
                set drain_length [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "ld" $cdfgData]] / $grid) } ])}] * $grid)]
                # If drain difussion length is less than minimum, set to Min
                if {[expr {($drain_length < $min_drain_length)}]} {
                    set drain_length $min_drain_length
                    puts "[eval format \"WARNING: Minimum Drain Diffusion Length is %g M\n\" $min_drain_length]"
                }; # if
                # If drain diffusion length is greater than maximum, set to Max
                if {[expr {($drain_length > $max_drain_length)}]} {
                    set drain_length $max_drain_length
                    puts "[eval format \"WARNING: Maximum Drain Diffusion Length is %g M\n\" $max_drain_length]"
                }; # if
                # Set Length
                iPDK_setParamValue "ld" $drain_length $cdfgData 0
                # Recalculate Area & Periphery
                ncsuAreaDrainCalc 
                ncsuPeriphDrainCalc
                return
}
#  ncsuMultiplierCallBack()                                  
#  sets: [m areas peripheries]                               
proc ncsuMultiplierCallBack {} {
    return [ncsuMultiplierCallBackDynamicImplementation]
}
proc ncsuMultiplierCallBackDynamicImplementation {} {
            global multiplier s
            set cdfgData [iPDK_getCurrentInst]
            set multiplier ""
                set multiplier [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "m" $cdfgData]] / 1.0) } ])}] * 1.0)]
                # If multiplier is less than 1, set to 1
                if {[expr {($multiplier < 1.0)}]} {
                    set multiplier 1.0
                    puts "[eval format \"WARNING: Minimum Multiplier is 1\n\"]"
                }; # if
                # If multiplier is greater than 500, set to 500
                if {[expr {($multiplier > 500.0)}]} {
                    set multiplier 500.0
                    puts "[eval format \"WARNING: Minimum Multiplier is 500\n\"]"
                }
                # Set Multiplier
                iPDK_setParamValue "m" $multiplier $cdfgData 0
                # Recalculate Areas & Peripheries
                ncsuAreaSourceCalc 
                ncsuAreaDrainCalc 
                ncsuPeriphSourceCalc 
                ncsuPeriphDrainCalc
                return
}
#                  SUBFUNCTIONS                              
# Calculates Source Area (As)
proc ncsuAreaSourceCalc {} {
    return [ncsuAreaSourceCalcDynamicImplementation]
}
proc ncsuAreaSourceCalcDynamicImplementation {} {
            global grid multiplier ncsuGlobalData poly_min_spacing s source_length width
            set cdfgData [iPDK_getCurrentInst]
            set grid ""
            set width ""
            set source_length ""
            set poly_min_spacing ""
            set multiplier ""
                set grid [dict get $ncsuGlobalData GridRes]
                set width [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "w" $cdfgData]] / $grid) } ])}] * $grid)]
                set source_length [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "ls" $cdfgData]] / $grid) } ])}] * $grid)]
                set poly_min_spacing [expr ([expr {round([ expr { ([readFloat [dict get $ncsuGlobalData PolyMinSpacing]] / $grid) } ]] * $grid)]
                set multiplier [expr {round([readFloat [iPDK_getParamValue "m" $cdfgData]])}]
                # As = W * Source Length (nm^2)
                # Varies based on the multiplier
                if {($multiplier <= 1)} {
                    return [iPDK_setParamValue "as" [ expr { ($width * $source_length) } ] $cdfgData 0]
                } elseif {($multiplier == 2)} {
                    return [iPDK_setParamValue "as" [ expr { (2 * ($width * $source_length)) } ] $cdfgData 0]
                } elseif {[and {[expr {($multiplier >= 3)}]} { [isEven $multiplier] }]} {
                    return [iPDK_setParamValue "as" [ expr { ((2 * ($width * $source_length)) + (((($multiplier - 4) / 2) + 1) * ($width * $poly_min_spacing))) } ] $cdfgData 0]
                } elseif {[and {[expr {($multiplier >= 3)}]} { [isOdd $multiplier] }]} {
                    return [iPDK_setParamValue "as" [ expr { (($width * $source_length) + (((($multiplier - 3) / 2) + 1) * ($width * $poly_min_spacing))) } ] $cdfgData 0]
                } elseif {true} {
                    puts "[eval format \"WARNING: ncsuAreaSourceCalc() FAILED\n\"]"
                    iPDK_setParamValue "as" 0.0 $cdfgData 0
                    return
                }
}
# Calculates Drain Area (Ad)
proc ncsuAreaDrainCalc {} {
    return [ncsuAreaDrainCalcDynamicImplementation]
}
proc ncsuAreaDrainCalcDynamicImplementation {} {
            global drain_length grid multiplier ncsuGlobalData poly_min_spacing s width
            set cdfgData [iPDK_getCurrentInst]
            set grid ""
            set width ""
            set drain_length ""
            set poly_min_spacing ""
            set multiplier ""
                set grid [dict get $ncsuGlobalData GridRes]
                set width [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "w" $cdfgData]] / $grid) } ])}] * $grid)]
                set drain_length [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "ld" $cdfgData]] / $grid) } ])}] * $grid)]
                set poly_min_spacing [expr ([expr {round([ expr { ([readFloat [dict get $ncsuGlobalData PolyMinSpacing]] / $grid) } ])}] * $grid)]
                set multiplier [expr {round([readFloat [iPDK_getParamValue "m" $cdfgData]])}]
                # As = W * Drain Length (nm^2)
                # Varies based on the multiplier
                if {($multiplier <= 1)} {
                    return [iPDK_setParamValue "ad" [ expr { ($width * $drain_length) } ] $cdfgData 0]
                } elseif {($multiplier == 2)} {
                    return [iPDK_setParamValue "ad" [ expr { ($width * $poly_min_spacing) } ] $cdfgData 0]
                } elseif {[and {[expr {($multiplier >= 3)}]} { [isEven $multiplier] }]} {
                    return [iPDK_setParamValue "ad" [ expr { (((($multiplier - 2) / 2) + 1) * ($width * $poly_min_spacing)) } ] $cdfgData 0]
                } elseif {[and {[expr {($multiplier >= 3)}]} { [isOdd $multiplier] }]} {
                    return [iPDK_setParamValue "ad" [ expr { (($width * $drain_length) + (((($multiplier - 3) / 2) + 1) * ($width * $poly_min_spacing))) } ] $cdfgData 0]
                } elseif {true} {
                    puts "[eval format \"WARNING: ncsuAreaDrainCalc() FAILED\n\"]"
                    iPDK_setParamValue "ad" 0.0 $cdfgData 0
                    return
                }
}
# Calculates Source Periphery (Ad)
proc ncsuPeriphSourceCalc {} {
    return [ncsuPeriphSourceCalcDynamicImplementation]
}
proc ncsuPeriphSourceCalcDynamicImplementation {} {
            global grid multiplier ncsuGlobalData poly_min_spacing s source_length width
            set cdfgData [iPDK_getCurrentInst]
            set grid ""
            set width ""
            set source_length ""
            set poly_min_spacing ""
            set multiplier ""
                set grid [dict get $ncsuGlobalData GridRes]
                set width [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "w" $cdfgData]] / $grid) } ])}] * $grid)]
                set source_length [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "ls" $cdfgData]] / $grid) } ])}] * $grid)]
                set poly_min_spacing [expr ([expr {round([ expr { ([readFloat [dict get $ncsuGlobalData PolyMinSpacing]] / $grid) } ])}] * $grid)]
                set multiplier [expr {round([readFloat [iPDK_getParamValue "m" $cdfgData]])}]
                # Ps = 2 * Source Length + Width (nm)
                # Varies based on the multiplier
                if {($multiplier <= 1)} {
                    return [iPDK_setParamValue "ps" [ expr { ($width + (2 * $source_length)) } ] $cdfgData 0]
                } elseif {($multiplier == 2)} {
                    return [iPDK_setParamValue "ps" [ expr { (2 * ($width + (2 * $source_length))) } ] $cdfgData 0]
                } elseif {[and {[expr {($multiplier >= 3)}]} { [isEven $multiplier] }]} {
                    return [iPDK_setParamValue "ps" [ expr { ((2 * ($width + (2 * $source_length))) + (((($multiplier - 4) / 2) + 1) * (2 * $poly_min_spacing))) } ] $cdfgData 0]
                } elseif {[and {[expr {($multiplier >= 3)}]} { [isOdd $multiplier] }]} {
                    return [iPDK_setParamValue "ps" [ expr { (($width + (2 * $source_length)) + (((($multiplier - 3) / 2) + 1) * (2 * $poly_min_spacing))) } ] $cdfgData 0]
                } elseif {true}
                    puts "[eval format \"WARNING: ncsuPeriphSourceCalc() FAILED\n\"]"
                    iPDK_setParamValue "ad" 0.0 $cdfgData 0
                    return
                }
}
# Calculates Drain Periphery (Ad)
proc ncsuPeriphDrainCalc {} {
    return [ncsuPeriphDrainCalcDynamicImplementation]
}
proc ncsuPeriphDrainCalcDynamicImplementation {} {
            global drain_length grid multiplier ncsuGlobalData poly_min_spacing s width
            set cdfgData [iPDK_getCurrentInst]
            set grid ""
            set width ""
            set drain_length ""
            set poly_min_spacing ""
            set multiplier ""
                set grid [dict get $ncsuGlobalData GridRes]
                set width [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "w" $cdfgData]] / $grid) } ])}] * $grid)]
                set drain_length [expr ([expr {round([ expr { ([readFloat [iPDK_getParamValue "ld" $cdfgData]] / $grid) } ])}] * $grid)]
                set poly_min_spacing [expr ([expr {round([ expr { ([readFloat [dict get $ncsuGlobalData PolyMinSpacing]] / $grid) } ])}] * $grid)]
                set multiplier [expr {round([readFloat [iPDK_getParamValue "m" $cdfgData]])}]
                # Pd = 2 * Drain Length + Width (nm)
                # Varies based on the multiplier
                if {($multiplier <= 1)} {
                    return [iPDK_setParamValue "pd" [ expr { ($width + (2 * $drain_length)) } ] $cdfgData 0]
                } elseif {($multiplier == 2)} {
                    return [iPDK_setParamValue "pd" [ expr { (2 * $poly_min_spacing) } ] $cdfgData 0]
                } elseif {[and {[expr {($multiplier >= 3)}]} { [isEven $multiplier] }]} {
                    return [iPDK_setParamValue "pd" [ expr { (((($multiplier - 2) / 2) + 1) * (2 * $poly_min_spacing)) } ] $cdfgData 0]
                } elseif {[and {[expr {($multiplier >= 3)}]} { [isOdd $multiplier] }]} {
                    return [iPDK_setParamValue "pd" [ expr { (($width + (2 * $drain_length)) + (((($multiplier - 3) / 2) + 1) * (2 * $poly_min_spacing))) } ] $cdfgData 0]
                } elseif {true} {
                    puts "[eval format \"WARNING: ncsuPeriphDrainCalc() FAILED\n\"]"
                    iPDK_setParamValue "ad" 0.0 $cdfgData 0
                    return
                    }
}

## CS ## Required procs

#readFloat
proc readFloat {input} {
    if {$input == ""} { return $input }
    set retVal $input
    if {[catch {set retVal [iPDK_engToSci $input] } ] } {
        return $input
    }
    return [expr double($retVal)]
}

#isOdd and isEven
proc isOdd {x} { 
    if {![string is int $x]} {
        puts "invalid syntax"
        return false
    } else {
        return [expr { [expr int($x) % 2] != 0 } ? true : false]
    }
}

proc isEven {x} { 
    if {![string is int $x]} {
        puts "invalid syntax"
        return false
    } else {
        return [expr { [expr int($x) % 2] == 0 } ? true : false]
    }
}

#and proc
proc and {args} {
    set res false
    foreach n $args {
        set res [uplevel 1 "expr {" $n "}" ]
        if {[regexp {>|<|=} $n]} {
            if {$res == 0 || $res == "false" || $res == "False"} {
                set res_val false
            } else {
                set res_val true
            }
        } else {
            if {$res == "" || $res == "nil" || $res == "\"nil\""} {
                set res_val false
            } else {
                set res_val $res
            }
        }
        set res $res_val
        if { $res == false || $res == "False" || $res == ""} {
            return $res
        }
    }
    return $res
}

