#!/bin/bash

subsystem=$1
testcasetype=$2

# vincode 保存位置
vincode_save="./vincode.save"

# echo $subsystem
# echo $testcasetype

if [ ! -f $vincode_save ]; then
    touch $vincode_save
    echo "create $vincode_save"
fi

usage(){
    echo "usage:"
    echo "./gen_vin.sh [subsystem] [testcasetype]"
    echo "subsystem - [1-OTASUBSYS, 2-MAPSENSOR, 3-SENSORAPP]"
    echo "testcasetype - [N-正常系, I-异常系]"
    echo "example: ./gen_vin.sh 1 N"
}

if [ "$subsystem" == "" ]; then
    usage
elif [ "$testcasetype" == "" ]; then
    usage
else
    ./vincode_generator -s $subsystem -t $testcasetype
    exit
fi
