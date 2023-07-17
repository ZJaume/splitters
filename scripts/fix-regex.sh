#!/bin/sh

for i in data_orig/*.srx; do
    name=${i/data_orig\///}
    echo $name

    cat $i \
    | perl -pe \
        's#<afterbreak> \+</afterbreak>#<afterbreak>\\ +</afterbreak>#g' \
    | perl -pe 's#\\ظ#ظ#g' \
    | perl -pe 's#\\Q\.\.\.\\E#\\.\\.\\.#g' \
    | perl -pe 's#(\[\\.+)-(–.*\])#$1\\-$2#g' \
    >src/splitters/data/$name
done
