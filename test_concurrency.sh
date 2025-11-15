#!/bin/bash

# Get number of requests from command line argument, default to 3
NUM_REQUESTS=${1:-3}

echo "========================================="
echo "Testing SEQUENTIAL vs CONCURRENT Behavior"
echo "========================================="
echo ""
echo "This script sends $NUM_REQUESTS requests in parallel."
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

# Send N requests in parallel (background processes)
for i in $(seq 1 $NUM_REQUESTS); do
    make_request $i &
done

# Wait for all background processes to complete
wait

echo ""
echo "All requests completed at: $(date +%H:%M:%S)"
echo ""
echo "========================================="
echo "EXPECTED BEHAVIOR:"
echo "========================================="
echo "SEQUENTIAL: All $NUM_REQUESTS requests take ~$((NUM_REQUESTS * 3)) seconds total (3s × $NUM_REQUESTS)"
echo "            Each request waits for the previous to finish"
echo ""
echo "CONCURRENT: All $NUM_REQUESTS requests take ~3 seconds total"
echo "            All process at the same time!"
echo "========================================="
