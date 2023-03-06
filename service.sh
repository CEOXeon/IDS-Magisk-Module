configfile=/storage/emulated/0/ids.conf
. /storage/emulated/0/ids.conf

watch -n 10 logcat | grep 'Password' > /storage/emulated/0/ids.log
