#!/bin/bash
# Demo script for Chequer network diagnostics

echo "🎯 Chequer Network Diagnostic Demo"
echo "===================================="
echo ""
echo "This demo will:"
echo "  1. Start a host server in the background"
echo "  2. Run a client test with live progress"
echo "  3. Display the fancy diagnostic report"
echo ""
echo "Press Enter to start..."
read

# Start host in background
echo "Starting host server..."
./target/release/chequer host --listen 127.0.0.1:7777 > /dev/null 2>&1 &
HOST_PID=$!
sleep 1

echo ""
echo "Running client diagnostic test..."
echo ""

# Run client test
./target/release/chequer client --connect 127.0.0.1:7777

# Cleanup
kill $HOST_PID 2>/dev/null

echo ""
echo "✅ Demo complete! Check chequer-report.json for detailed results."
