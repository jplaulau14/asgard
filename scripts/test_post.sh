#!/bin/bash

echo "========================================="
echo "Testing Concurrent POST Requests"
echo "========================================="
echo ""

# Function to make a POST request
make_post() {
    local id=$1
    local response=$(curl -s -X POST http://localhost:8000/echo -d "Message $id")
    echo "[Request $id] Response: $response"
}

echo "Sending 5 POST requests concurrently..."
echo ""

# Send 5 requests in parallel
for i in {1..5}; do
    make_post $i &
done

# Wait for all to complete
wait

echo ""
echo "========================================="
echo "All POST requests completed!"
echo "========================================="
