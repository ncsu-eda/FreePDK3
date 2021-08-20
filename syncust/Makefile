# 3nm FreePDK(TM) Makefile
#
# Copyright (c) 2021, North Carolina State University
# All Rights Reserved.
#
# Please see the file LICENSE included with this distribution for license.
# You may not use these files except in compliance with the License.


TECHDIR = NCSU_TechLib_FreePDK3
NAME = V1

.PHONY: tech
tech: $(TECHDIR)/tech.db

$(TECHDIR)/tech.db: 
	custom_compiler -gui false -log tech.log -command 'source NCSU_TechLib_FreePDK3/autoLoad.file ; source Tcl/utils/updateTech.tcl ; exit -force true'
	custom_compiler -gui false -log cdf.log -command 'source NCSU_TechLib_FreePDK3/autoLoad.file ; source cdf/loadAll.tcl ; exit -force true'



clean:
	-rm -rf synopsys_custom
	-rm -f $(TECHDIR)/tech.db
	-rm -f $(TECHDIR)/data.dm
	-find . -name \*.png -exec rm {} \;

