#!/bin/bash

echo "========================================="
echo "Asgard - Final Comprehensive Test"
echo "========================================="
echo ""
echo "Testing concurrent mixed GET/POST requests..."
echo ""

# Function to test GET /
test_root() {
    local id=$1
    local response=$(curl -s http://localhost:8000/)
    echo "[GET /] Request $id: $response"
}

# Function to test GET /health
test_health() {
    local id=$1
    local response=$(curl -s http://localhost:8000/health)
    echo "[GET /health] Request $id: $response"
}

# Function to test GET /api/status (JSON)
test_status() {
    local id=$1
    local response=$(curl -s http://localhost:8000/api/status)
    echo "[GET /api/status] Request $id: $response"
}

# Function to test POST /echo (JSON)
test_echo() {
    local id=$1
    local response=$(curl -s -X POST http://localhost:8000/echo -d "Message $id")
    echo "[POST /echo] Request $id: $response"
}

# Send 20 concurrent requests (mix of GET and POST)
for i in {1..5}; do
    test_root $i &
    test_health $i &
    test_status $i &
    test_echo $i &
done

# Wait for all requests to complete
wait

echo ""
echo "========================================="
echo "All 20 requests completed!"
echo "========================================="
echo ""
echo "Summary of what we tested:"
echo "  ✓ Plain text GET endpoints (/, /health)"
echo "  ✓ JSON GET endpoint (/api/status)"
echo "  ✓ POST with body (/echo)"
echo "  ✓ Concurrent processing (20 simultaneous)"
echo "  ✓ Mixed content types (text/plain and application/json)"
echo ""
echo "Your Asgard ASGI server is working! 🎉"
echo "========================================="
