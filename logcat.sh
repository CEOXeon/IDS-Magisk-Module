data_file=/sdcard/ids/data.sh

counter=1
while read -r line; do
    varname="line_$counter"
    
    #checking every 30 seconds if sensitive data is present in logs
    watch -n 30 logcat | grep $varname > $log_file && continue
    counter=($counter(+1))
done < "$data_file"