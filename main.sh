
MODDIR=${0%/*}
config_file=/sdcard/ids/ids.sh
config_dir=/sdcard/ids
log_file=/sdcard/ids/ids.log

if [ ! -d $config_dir ]; then
  mkdir $config_dir
fi

if [ ! -f $config_file ]; then
    touch $config_file
fi

./logcat.sh && continue


#dns server checks
dns_server=$(nslookup google.com | awk '/^Server:/ {print $2}')
echo "Used DNS-Server: $dns_server"
