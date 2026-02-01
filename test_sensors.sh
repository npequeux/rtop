#!/bin/bash

echo "=== Testing temperature sensor detection ==="
echo ""

# Test with sensors command
echo "1. Checking with 'sensors' command:"
if command -v sensors &> /dev/null; then
    sensors 2>&1 | head -30
else
    echo "   'sensors' command not found (install lm-sensors)"
fi

echo ""
echo "2. Checking /sys/class/thermal:"
if [ -d "/sys/class/thermal" ]; then
    for zone in /sys/class/thermal/thermal_zone*/; do
        if [ -f "${zone}temp" ]; then
            temp=$(cat "${zone}temp" 2>/dev/null)
            type=$(cat "${zone}type" 2>/dev/null)
            echo "   $type: $((temp/1000))°C"
        fi
    done
else
    echo "   No thermal zones found"
fi

echo ""
echo "3. Checking /sys/class/hwmon:"
if [ -d "/sys/class/hwmon" ]; then
    for hwmon in /sys/class/hwmon/hwmon*/; do
        if [ -f "${hwmon}name" ]; then
            name=$(cat "${hwmon}name" 2>/dev/null)
            echo "   Found hwmon: $name"
            for temp_input in "${hwmon}"temp*_input; do
                if [ -f "$temp_input" ]; then
                    temp=$(cat "$temp_input" 2>/dev/null)
                    echo "      Temperature: $((temp/1000))°C"
                fi
            done
        fi
    done
else
    echo "   No hwmon devices found"
fi

echo ""
echo "4. Running rtop to check detection:"
echo "   (will display for 3 seconds)"
timeout 3 ./target/release/rtop || echo "   Application terminated"
