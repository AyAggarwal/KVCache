#!/bin/bash

# Configuration
TARGETS_FILE="targets.txt"
DURATION="10s"
OUTPUT_DIR="vegeta_results"
RATES=( $(seq 100 100 10000) ) # Rates from 100 to 10,000, step 100
CSV_OUTPUT="vegeta_results.csv"

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

# Check if targets.txt exists
if [ ! -f "$TARGETS_FILE" ]; then
    echo "Error: $TARGETS_FILE not found. Please create it with your Vegeta targets."
    exit 1
fi

# Check if vegeta is installed
if ! command -v vegeta &> /dev/null; then
    echo "Error: Vegeta is not installed. Please install it first."
    exit 1
fi

# Initialize CSV file with headers
echo "rate,latency_mean,latency_p95,latency_p99,success_rate" > "$CSV_OUTPUT"

# Run Vegeta attacks for each rate
for rate in "${RATES[@]}"; do
    echo "Running Vegeta attack at $rate requests/second..."
    
    # Run the attack and output JSON
    JSON_OUTPUT="$OUTPUT_DIR/result_$rate.json"
    vegeta attack -targets="$TARGETS_FILE" -rate="$rate" -duration="$DURATION" | \
        vegeta report -type=json > "$JSON_OUTPUT"
    
    # Check if the attack succeeded
    if [ $? -ne 0 ]; then
        echo "Error: Vegeta attack failed for rate $rate"
        continue
    fi

    # Extract metrics from JSON using jq and append to CSV
    latency_mean=$(jq -r '.latencies.mean' "$JSON_OUTPUT")
    latency_p95=$(jq -r '.latencies["95th"]' "$JSON_OUTPUT")
    latency_p99=$(jq -r '.latencies["99th"]' "$JSON_OUTPUT")
    success_rate=$(jq -r '.success' "$JSON_OUTPUT")

    # Convert latencies from nanoseconds to milliseconds for easier reading
    latency_mean_ms=$(echo "scale=3; $latency_mean / 1000000" | bc)
    latency_p95_ms=$(echo "scale=3; $latency_p95 / 1000000" | bc)
    latency_p99_ms=$(echo "scale=3; $latency_p99 / 1000000" | bc)

    # Append to CSV
    echo "$rate,$latency_mean_ms,$latency_p95_ms,$latency_p99_ms,$success_rate" >> "$CSV_OUTPUT"
done

echo "Benchmarking complete. Results saved to $CSV_OUTPUT"
echo "Raw JSON results are in $OUTPUT_DIR/"