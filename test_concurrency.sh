#!/bin/bash

echo "========================================="
echo "Testing SEQUENTIAL vs CONCURRENT Behavior"
echo "========================================="
echo ""
echo "This script sends 3 requests in parallel."
echo "Watch the timestamps to see if they're processed:"
echo "  - SEQUENTIALLY (one waits for another to finish)"
echo "  - CONCURRENTLY (all processing at the same time)"
echo ""
echo "Starting requests at: $(date +%H:%M:%S)"
echo ""

# Function to make a request and show timing
make_request() {
    local id=$1
    local start=$(date +%s)
    echo "[Request $id] Starting at $(date +%H:%M:%S)"

    response=$(curl -s http://localhost:8000/)

    local end=$(date +%s)
    local duration=$((end - start))
    echo "[Request $id] Finished at $(date +%H:%M:%S) (took ${duration}s) - Response: $response"
}

# Send 3 requests in parallel (background processes)
make_request 1 &
make_request 2 &
make_request 3 &

# Wait for all background processes to complete
wait

echo ""
echo "All requests completed at: $(date +%H:%M:%S)"
echo ""
echo "========================================="
echo "EXPECTED BEHAVIOR:"
echo "========================================="
echo "SEQUENTIAL (current): All 3 requests take ~9 seconds total (3s + 3s + 3s)"
echo "                      Each request waits for the previous to finish"
echo ""
echo "CONCURRENT (after Task 5): All 3 requests take ~3 seconds total"
echo "                           All process at the same time!"
echo "========================================="
