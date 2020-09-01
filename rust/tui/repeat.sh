#!/bin/bash

a=0
while [ $a -lt 10 ];
do
echo "<$a>"
sleep 1
((a++))
done