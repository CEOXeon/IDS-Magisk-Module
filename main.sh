#!/system/bin/sh

MODDIR=${0%/*}
configfile=/storage/emulated/0/ids.sh

if [ ! -f $configfile ]; then
    touch $configfile
fi

. $configfile

counter=1
while read -r line; do
    varname="line_$counter"
    declare "$varname=$line"
    counter=$((counter+1))
done < "$configfile"

#checking every 30 seconds if sensitive data is present in logs
watch -n 30 logcat | grep $line_1 > /storage/emulated/0/ids.log && watch -n 30 logcat | grep $line_2 > /storage/emulated/0/ids.log && watch -n 30 logcat | grep $line_3 > /storage/emulated/0/ids.log && continue

