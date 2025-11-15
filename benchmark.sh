#!/bin/bash

echo "========================================="
echo "Asgard Performance Benchmark"
echo "========================================="
echo ""

NUM_REQUESTS=1000
echo "Sending $NUM_REQUESTS requests..."
echo ""

START=$(date +%s)

for i in $(seq 1 $NUM_REQUESTS); do
    curl -s http://localhost:8000/ > /dev/null &
done

wait

END=$(date +%s)
DURATION=$((END - START))

if [ $DURATION -eq 0 ]; then
    DURATION=1  # Avoid division by zero
fi

RPS=$((NUM_REQUESTS / DURATION))

echo ""
echo "========================================="
echo "Results:"
echo "========================================="
echo "Total requests: $NUM_REQUESTS"
echo "Total time: ${DURATION}s"
echo "Requests per second: ~$RPS"
echo "========================================="
echo ""
echo "For comparison:"
echo "  - Simple Node.js server: ~5,000-10,000 RPS"
echo "  - Uvicorn (Python): ~10,000-20,000 RPS"
echo "  - Production Rust (optimized): ~50,000-100,000 RPS"
echo "========================================="
