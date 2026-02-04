#!/bin/bash

echo "=== Performance Testing ==="
echo ""

# Test 1: Binary size
echo "1. Binary Size:"
ls -lh target/release/rtop | awk '{print "   "$5}'

# Test 2: Startup time
echo ""
echo "2. Startup Time (5 runs):"
for i in {1..5}; do
    /usr/bin/time -f "   Run $i: %E elapsed, %M KB max memory" timeout 1 ./target/release/rtop 2>&1 | grep "Run"
done

# Test 3: Memory usage
echo ""
echo "3. Memory Usage Test (2 seconds):"
./target/release/rtop &
PID=$!
sleep 0.5
ps -p $PID -o rss,vsz,cmd 2>/dev/null | tail -1 | awk '{print "   RSS: "$1" KB, VSZ: "$2" KB"}'
kill $PID 2>/dev/null
wait $PID 2>/dev/null

# Test 4: CPU usage
echo ""
echo "4. CPU Usage Test (monitoring for 3 seconds):"
./target/release/rtop &
PID=$!
sleep 1
top -b -n 3 -d 1 -p $PID 2>/dev/null | grep rtop | awk '{sum+=$9; count++} END {if(count>0) print "   Average CPU: " sum/count "%"}'
kill $PID 2>/dev/null
wait $PID 2>/dev/null

echo ""
echo "=== Optimization Summary ==="
echo "✓ LTO enabled (fat)"
echo "✓ Single codegen unit"
echo "✓ Symbols stripped"
echo "✓ Panic=abort"
echo "✓ Minimal tokio features"
echo "✓ Pre-allocated vectors"
echo "✓ Reduced polling frequency"
echo "✓ Less frequent disk/process updates"
