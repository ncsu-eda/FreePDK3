
set libName  NCSU_TechLib_FreePDK3
set filename techfiles/NCSU_TechLib_FreePDK3_CC.tf
set tech [db::importTechFile $filename -libName $libName]
oa::save $tech
