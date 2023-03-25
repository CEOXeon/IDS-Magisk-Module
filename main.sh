#!/system/bin/sh

MODDIR=${0%/*}
config_file=/sdcard/ids/ids.sh
config_dir=/sdcard/ids
log_file=/sdcard/ids/ids.log
data_file=/sdcard/ids/data.sh

if [ ! -d $config_dir ]; then
  mkdir $config_dir
fi

if [ ! -f $config_file ]; then
    touch $config_file
fi

#. /$configfile

counter=1
while read -r line; do
    varname="line_$counter"
    
    #checking every 30 seconds if sensitive data is present in logs
    watch -n 30 logcat | grep $varname > $log_file && continue
    counter=$((counter+1))
done < "$data_file"

#dns server checks
dns_server=$(nslookup google.com | awk '/^Server:/ {print $2}')
echo "Used DNS-Server: $dns_server"
