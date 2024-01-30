#!/bin/bash

# TODO: run clear after timeout and wipe clipboard
# condition fails when unset **and** when equal to ""
while [[ -z "${BW_SESSION}" ]]
do
  read -s -p "Master Password: "
  BW_SESSION=$(echo ${REPLY} | bw --raw unlock 2>/dev/null) # stderr has `mac failed.`
  unset REPLY
done

echo && echo Session ID: ${BW_SESSION}

while true
do
  read -p "BW~$ " -a arr
  case ${arr[0]} in
    pass | card | name)
      # stderr node deprecation warning on MacOS
      RAW=$(bw --raw --session "${BW_SESSION}" list items --search "${arr[1]}" 2>/dev/null)
      bw-tool-helper "${RAW}" "${arr[0]}"
      ;;
    clear | c)
      clear
      ;;
    quit| q)
      exit 0
    ;;
  esac
done
