#! /bin/bash

pids=()

just p 9001 &
last_pid=$!
pids+=($last_pid)

just p 9002 &
last_pid=$!
pids+=($last_pid)

just p 9003 &
last_pid=$!
pids+=($last_pid)

sleep 2
for i in ${pids}; do
    echo "killing $i";
    kill -KILL $i
done
