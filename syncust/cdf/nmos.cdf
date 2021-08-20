# 3nm FreePDK(TM) NMOS CDF
#
# Copyright (c) 2021, North Carolina State University
# All Rights Reserved.
#
# Please see the file LICENSE included with this distribution for license.
# You may not use these files except in compliance with the License.




proc loadParams {libName {cellName {}} {overwrite 1} {forceMaster 0}} {
    set data [oa::LibFind $libName]
    if { "" == $data } {
        set data [oa::LibCreate $libName ./$libName]
    }
    if { "" != $cellName } {
        set data [dm::findCell $cellName -libName $libName]
    }
    if { $overwrite } {
        db::purgeCDF $data -forceMaster $forceMaster -removeCDF 1
    }

    # Top level properties
    db::setNetlistInfo promptWidth                                             \
        -value {175}                                                           \
        -type integer                                                          \
        -data $data                                                            \
        -section form                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo fieldHeight                                             \
        -value {35}                                                            \
        -type integer                                                          \
        -data $data                                                            \
        -section form                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo fieldWidth                                              \
        -value {350}                                                           \
        -type integer                                                          \
        -data $data                                                            \
        -section form                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo buttonFieldWidth                                        \
        -value {340}                                                           \
        -type integer                                                          \
        -data $data                                                            \
        -section form                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo formInitProc                                            \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section form                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo doneProc                                                \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section form                                                          \
        -forceMaster $forceMaster                                              

    # Parameter defintions
        db::createParamDef model                                               \
        -data          $data                                                   \
        -prompt        {Model name}                                            \
        -defValue      {nmos}                                                  \
        -type          string                                                  \
        -parseAsNumber 0                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('model)}                     \
        -storeDefault  1                                                       \
        -propList      {nil}                                                   

        db::createParamDef nfin                                                   \
        -data          $data                                                   \
        -prompt        {Number of Fins}                                        \
        -defValue      {2}                                                     \
        -type          string                                                  \
        -callback      {ncsuNFinCallBack()}                                    \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('nfin)}                         \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef w                                                   \
        -data          $data                                                   \
        -prompt        {Width}                                                 \
        -type          string                                                  \
        -units         lengthMetric                                            \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('w)}                         \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef l                                                   \
        -data          $data                                                   \
        -prompt        {Length}                                                \
        -type          string                                                  \
        -units         lengthMetric                                            \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('l)}                         \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef asej                                                  \
        -data          $data                                                   \
        -prompt        {Source diffusion area}                                 \
        -defValue      {6.08e-16}                                              \
        -type          string                                                  \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -editable      {nil}                                                   \
        -display       {artParameterInToolDisplay('asej)}                        \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef adej                                                  \
        -data          $data                                                   \
        -prompt        {Drain diffusion area}                                  \
        -defValue      {6.08e-16}                                              \
        -type          string                                                  \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -editable      {nil}                                                   \
        -display       {artParameterInToolDisplay('adej)}                        \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef psej                                                  \
        -data          $data                                                   \
        -prompt        {Source diffusion periphery}                            \
        -defValue      {168n}                                                  \
        -type          string                                                  \
        -units         lengthMetric                                            \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -editable      {nil}                                                   \
        -display       {artParameterInToolDisplay('psej)}                        \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef pdej                                                  \
        -data          $data                                                   \
        -prompt        {Drain diffusion periphery}                             \
        -defValue      {168n}                                                  \
        -type          string                                                  \
        -units         lengthMetric                                            \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -editable      {nil}                                                   \
        -display       {artParameterInToolDisplay('pdej)}                        \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef nrd                                                 \
        -data          $data                                                   \
        -prompt        {Drain diffusion res squares}                           \
        -type          string                                                  \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('nrd)}                       \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef nrs                                                 \
        -data          $data                                                   \
        -prompt        {Source diffusion res squares}                          \
        -type          string                                                  \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('nrs)}                       \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef ld                                                  \
        -data          $data                                                   \
        -prompt        {Drain diffusion length}                                \
        -type          string                                                  \
        -units         lengthMetric                                            \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('ld)}                        \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef ls                                                  \
        -data          $data                                                   \
        -prompt        {Source diffusion length}                               \
        -type          string                                                  \
        -units         lengthMetric                                            \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('ls)}                        \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef trise                                               \
        -data          $data                                                   \
        -prompt        {Temp rise from ambient}                                \
        -type          string                                                  \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('trise)}                     \
        -use           {!cdfgData->triseSpec ||                        cdfgData->triseSpec->value == "trise"}\
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef region                                              \
        -data          $data                                                   \
        -prompt        {Estimated operating region}                            \
        -defValue      { }                                                     \
        -type          cyclic                                                  \
        -choices       {" " "off" "triode" "sat" "subth"}                      \
        -parseAsNumber 0                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('region)}                    \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef degradation                                         \
        -data          $data                                                   \
        -prompt        {Hot-electron degradation}                              \
        -defValue      {no}                                                    \
        -type          cyclic                                                  \
        -choices       {"no" "yes"}                                            \
        -parseAsNumber 0                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('degradation)}               \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef geo                                                 \
        -data          $data                                                   \
        -prompt        {Source/drain selector}                                 \
        -type          string                                                  \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('geo)}                       \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef rdc                                                 \
        -data          $data                                                   \
        -prompt        {Additional drain resistance}                           \
        -type          string                                                  \
        -units         resistance                                              \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('rdc)}                       \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef rsc                                                 \
        -data          $data                                                   \
        -prompt        {Additional source resistance}                          \
        -type          string                                                  \
        -units         resistance                                              \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('rsc)}                       \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef sa                                                  \
        -data          $data                                                   \
        -prompt        {Dist. OD & poly(one side)}                             \
        -type          string                                                  \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('sa)}                        \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef sb                                                  \
        -data          $data                                                   \
        -prompt        {Dist. OD & poly(other side)}                           \
        -type          string                                                  \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('sb)}                        \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef sd                                                  \
        -data          $data                                                   \
        -prompt        {Dist. betn neighbour fingers}                          \
        -type          string                                                  \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('sd)}                        \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef off                                                 \
        -data          $data                                                   \
        -prompt        {Device initially off}                                  \
        -defValue      {0}                                                     \
        -type          boolean                                                 \
        -parseAsNumber 0                                                       \
        -parseAsPEL    0                                                       \
        -display       {artParameterInToolDisplay('off)}                       \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef Vds                                                 \
        -data          $data                                                   \
        -prompt        {Drain source initial voltage}                          \
        -type          string                                                  \
        -units         voltage                                                 \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('Vds)}                       \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef Vgs                                                 \
        -data          $data                                                   \
        -prompt        {Gate source initial voltage}                           \
        -type          string                                                  \
        -units         voltage                                                 \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('Vgs)}                       \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef Vbs                                                 \
        -data          $data                                                   \
        -prompt        {Bulk source initial voltage}                           \
        -type          string                                                  \
        -units         voltage                                                 \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('Vbs)}                       \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef dtemp                                               \
        -data          $data                                                   \
        -prompt        {Temperature difference}                                \
        -type          string                                                  \
        -parseAsNumber 1                                                       \
        -parseAsPEL    1                                                       \
        -display       {artParameterInToolDisplay('dtemp)}                     \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

        db::createParamDef modelName                                           \
        -data          $data                                                   \
        -prompt        {modelName}                                             \
        -defValue      {nfet}                                                  \
        -type          string                                                  \
        -parseAsNumber 0                                                       \
        -parseAsPEL    0                                                       \
        -display       {nil}                                                   \
        -storeDefault  0                                                       \
        -propList      {nil}                                                   

    # Label display section
    db::setNetlistInfo modelLabelSet                                           \
        -value {vto kp gamma}                                                  \
        -type string                                                           \
        -data $data                                                            \
        -section display                                                       \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo opPointLabelSet                                         \
        -value {id vgs vds}                                                    \
        -type string                                                           \
        -data $data                                                            \
        -section display                                                       \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo paramLabelSet                                           \
        -value {-model l w}                                                    \
        -type string                                                           \
        -data $data                                                            \
        -section display                                                       \
        -forceMaster $forceMaster                                              

    # Netlister specific information for spectre
    db::setNetlistInfo modelParamExprList                                      \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister spectre                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo optParamExprList                                        \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister spectre                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo opParamExprList                                         \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister spectre                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo stringParameters                                        \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister spectre                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo componentName                                           \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister spectre                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo netlistProcedure                                        \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister spectre                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo propMapping                                             \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister spectre                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo namePrefix                                              \
        -value {}                                                              \
        -type string                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister spectre                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo otherParameters                                         \
        -value {(model)}                                                       \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister spectre                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo instParameters                                          \
        -value {(nfin w l asej adej psej pdej nrd nrs ld ls trise region degradation geo rdc rsc sa sb sd)}\
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister spectre                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo termOrder                                               \
        -value {(D G S)}                                                     \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister spectre                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo termMapping                                             \
        -value {(nil D \:d G \:g S \:s)}                                 \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister spectre                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    # Netlister specific information for hspiceD
    db::setNetlistInfo opParamExprList                                         \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister hspiceD                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo optParamExprList                                        \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister hspiceD                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo termMapping                                             \
        -value {(nil D \,D G \,G S \,S)}                                 \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister hspiceD                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo netlistProcedure                                        \
        -value {hspiceDCompPrim}                                               \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister hspiceD                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo instParameters                                          \
        -value {(nfin w l adej asej pdej psej nrd nrs rdc rsc off Vds Vgs Vbs dtemp geo)} \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister hspiceD                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo otherParameters                                         \
        -value {(model)}                                                       \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister hspiceD                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo componentName                                           \
        -value {nmos}                                                          \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister hspiceD                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo termOrder                                               \
        -value {(D G S)}                                                     \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister hspiceD                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo dataAccessMap                                           \
        -value {((IDC (D "id")))}                                              \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister hspiceD                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo propMapping                                             \
        -value {(nil vds Vds vgs Vgs vbs Vbs model modelName)}                 \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister hspiceD                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo namePrefix                                              \
        -value {M}                                                             \
        -type string                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister hspiceD                                                     \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    # Netlister specific information for auLvs
    db::setNetlistInfo deviceTerminals                                         \
        -value {}                                                              \
        -type string                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auLvs                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo otherParameters                                         \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auLvs                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo propMapping                                             \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auLvs                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo netlistProcedure                                        \
        -value {ansLvsCompPrim}                                                \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auLvs                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo instParameters                                          \
        -value {(m l w)}                                                       \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auLvs                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo componentName                                           \
        -value {nmos}                                                          \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auLvs                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo termOrder                                               \
        -value {(D G S)}                                                     \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auLvs                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo permuteRule                                             \
        -value {(p D S)}                                                       \
        -type string                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auLvs                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo namePrefix                                              \
        -value {M}                                                             \
        -type string                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auLvs                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    # Netlister specific information for auCdl
    db::setNetlistInfo dollarEqualParams                                       \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auCdl                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo dollarParams                                            \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auCdl                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo otherParameters                                         \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auCdl                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo netlistProcedure                                        \
        -value {ansCdlCompPrim}                                                \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auCdl                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo instParameters                                          \
        -value {(m L W)}                                                       \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auCdl                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo componentName                                           \
        -value {nmos}                                                          \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auCdl                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo termOrder                                               \
        -value {(D G S)}                                                     \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auCdl                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo propMapping                                             \
        -value {(nil L l W w)}                                                 \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auCdl                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo namePrefix                                              \
        -value {M}                                                             \
        -type string                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auCdl                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo modelName                                               \
        -value {NFET}                                                          \
        -type string                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister auCdl                                                       \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    # Netlister specific information for ams
    db::setNetlistInfo extraTerminals                                          \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo componentName                                           \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo excludeParameters                                       \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo arrayParameters                                         \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo stringParameters                                        \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo referenceParameters                                     \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo netlistProcedure                                        \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo propMapping                                             \
        -value {nil}                                                           \
        -type symbol                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo namePrefix                                              \
        -value {}                                                              \
        -type string                                                           \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo isPrimitive                                             \
        -value {(t)}                                                           \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo enumParameters                                          \
        -value {(region)}                                                      \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo termOrder                                               \
        -value {(D G S)}                                                     \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo termMapping                                             \
        -value {(nil D \:d G \:g S \:s)}                                 \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo instParameters                                          \
        -value {(nfin w l asej adej psej pdej nrd nrs ld ls trise model)}                 \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    db::setNetlistInfo otherParameters                                         \
        -value {(model)}                                                       \
        -type list                                                             \
        -data $data                                                            \
        -section netlist                                                       \
        -netlister ams                                                         \
        -info simInfo                                                          \
        -forceMaster $forceMaster                                              

    # Netlister specific information for UltraSim
    db::saveCDF $data -forceMaster $forceMaster
}

if {[oa::CellDMDataExists NCSU_TechLib_FreePDK3 nmos]} {
  oa::CellDMDataDestroy NCSU_TechLib_FreePDK3 nmos
}

loadParams NCSU_TechLib_FreePDK3 nmos
