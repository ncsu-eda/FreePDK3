FreePDK3 Release Notes
======================

## Version 1.0 (2021-08-30)

* Included in this release
  * Technology library and display resources for Synopsys Custom Compiler
      (Tested with Custom Compiler Q-2020.03-SP2)
  * IC Validator DRC & LVS Rules (Tested with ICV R-2020.09)
  * StarRC extraction Models (Tested with StarRC R-2020.09)
  * HSPICE Simulation Models (Tested with HSPICE Q-2020.03-SP2)
* Issues with this release
  * Design Rules
    * The ANTENNA rule is implemented, but no protection-diode structure 
	     exists.  Therefore, some other means to fix the violation would be 
	     required for long run lengths (such as "stapling" to a lower metal 
	     layer).  We plan to release an updated kit with a protection-diode.  
      Until then, this rule can be disabled by editing the file 
      icv/FreePDK3_main.drc.rs by inserting comment characters "//" in 
	     front of the line that includes FreePDK3.antenna.drc.rs.
	* Preliminary double patterning rules have been added for metal layers
	  1-3 and 7-9.  Rule Mx.7 decomposes each layer into two masks and
	  stores the result in a file called m#_dpt_output.gds.  Rule Mx.8 
	  checks to ensure that density requirements are met.  These rules 
	  are otherwise undocumented, buecause we are still tuning them and 
	  expect them to change in a future release.
  * Circuit Simulation Models
    * Preliminary HSPICE models have been added, based on a simple 
	     Sentaurus simulation.  This model includes a single nanosheet wrapped
	     in gate metal with no source or drain regions.  Our simulations are
	     showing much better performance than we would have expected, so please
	     use these models only as a first glimpse of a gate-all-around FET.  
	     More work is needed to properly tune these models.  Please contact us
	     if you would like to help with this task.
  * Parasitic Extraction Models
    * StarRC extraction models are also are provided for a typical process.
      We anticipate supporting corner models in a future release.
	* StarRC models are complete down to the V0A layer, but the lower layers
	  need additional work.  In particular, we anticipate one or more of
	  the following changes in a future release:
	  * Separate GATE layers in ICV LVS rules for device and field,
	    with the LAYER_TYPE properly set, to avoid double-counting the GATE capacitance (Rule 2, p. 8-13 in StarRC User Guide)
	  * a MULTIGATE statement to model the gate all around the nanosheet 
	    (p. 12-4 in StarRC User Guide)
	  * TRENCH_CONTACT layer-types to properly model the M0A layer
	  * Proper modeling of the conformal dielectric of the buried power-rail
     to capture substrate coupling capacitance
