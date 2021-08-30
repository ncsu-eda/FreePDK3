FreePDK3 <sup>TM</sup> Predictive Process Design Kit
=====================================================
version 1.0 (2021-08-30)

Copyright (c) 2021 North Carolina State University,
All Rights Reserved.

Please see the file LICENSE in this directory for license.
You may not use these files except in compliance with the License.

## Welcome to the FreePDK <sup>TM</sup> 3nm Free, Open-Source Process Design Kit 

This initiative is brought to you by NC State Univeristy and Synopsys.

This version of the kit was created by the following at NC State University:
* Sushant Sadangi              - Design Rules, Layer Stack, and ICV Rules
* Viswanatha Pasumarthy        - Star-RC Models
* W. Shepherd Pitts            - HSPICE Models
* W. Rhett Davis               - Custom Compiler Support and Project Management

Many thanks to the following at Synopsys for technical and financial support:
* Ron Duncan                      - Process technology advice and ICV Support
* Luis Francisco & Yen-Sung Chen  - ICV Rules Templates and Coding Support
* Olaf Schneider                  - Custom Compiler PDK and Scripting Support
* Jonathan White & Patrick Haspel - Project Management


## Quick Design Kit Usage Instructions

  1) Create or modify the file lib.defs in your current directory
     and ensure that it includes the following line:

          INCLUDE $PDK_DIR/syncust/lib.defs

     ...where $PDK_DIR is replaced with the path to the directory
     containing this file.  Note that the environmnet variable
     PDK_DIR does not need to be set.

  2) Source your setup scripts for Synopsys Custom Compiler, IC Validator,
     Star-RC, HSPICE, and Custom WaveView

  3) Start Synopsys Custom Compiler with the command `custom_compiler &`

## Contents 

| Directory  | Contents                                                       |
| ---------- | -------------------------------------------------------------- |
| syncust/   | Technology libraries and scripts for Synopsys Custom Compiler  |
| icv/       | DRC and LVS rules for Synopsys IC Validator                    |
| starrc/    | Parasitic extraction models for Synopsys StarRC                |
| hspice/    | Simulation models for Synopsys HSPICE                          |
| examples/  | Example library with layouts and schematics                    |
| doc/       | Design Rule Manual and Release Notes                           |


More documentation and tutorials for this kit
can be found at <http://www.eda.ncsu.edu/FreePDK>

*Please send all questions and comments to <eda_help@ncsu.edu>*


