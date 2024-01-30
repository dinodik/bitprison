#!/bin/bash

if [[ ! -v BW_SESSION ]]; then
  read -s -p "Master Password: "
  BW_SESSION=$(echo $REPLY | bw --raw unlock 2>/dev/null) # stderr has `mac failed.`
  echo $BW_SESSION
  unset REPLY
fi

while true
do
  read -p "BW~$ " -a arr
  case ${arr[0]} in
    pass | card | name)
      RAW=$(bw --raw list items --search ${arr[1]})
      target/debug/bw-tool $RAW ${arr[0]}
      ;;
    clear | c)
      clear
      ;;
    quit| q)
      exit 0
    ;;
  esac
done
