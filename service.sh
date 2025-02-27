# Do NOT assume where your module will be located.
# ALWAYS use $MODDIR if you need to know where this script and module is placed.
# This will make sure your module will still work if Magisk change its mount point in the future
MODDIR=${0%/*}

# This script will be executed in late_start service mode. More info in the main Magisk thread
#Modify the necessary varible MODDIR with magisk provided  varible eg. /data/adb/modules/IDS-Magisk-Module
#sed  -i -e "s/^MODDIR.*$/MODDIR=${MODDIR}/" "$MODDIR/IDS-Magisk-Module"

MAGMOD=$MODDIR

#Let boot status 

# Wait till boot has completed'
(while [ "$(getprop sys.boot_completed | tr -d '\r')" != "1" ]; do sleep 20; done


	if [ `getprop sys.boot_completed` = 1 ]; then 
	
	
		#Reserved for crond schedule
		
		
		
		#prevent recursive run
		su -c sh "$MAGMOD/main.sh"
		
	fi
	
)
