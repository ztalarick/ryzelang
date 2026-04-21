#!/bin/bash

# run_tests.sh - Ryzelang Expectation Test Runner

# ANSI color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

UPDATE_EXPECTATIONS=false
if [[ "$1" == "--bless" ]] || [[ "$1" == "-u" ]]; then
    UPDATE_EXPECTATIONS=true
    shift
fi

# Build the CLI
echo -e "${YELLOW}Building Ryzelang CLI...${NC}"
cargo build --package ryzelang-cli --quiet

FAILED_TESTS=()
PASSED_COUNT=0
TOTAL_COUNT=0

# Ensure tests directory exists
if [ ! -d "tests" ]; then
    echo -e "${RED}Error: tests/ directory not found.${NC}"
    exit 1
fi

# Loop through all .ryze files in tests/
for test_file in tests/*.ryze; do
    ((TOTAL_COUNT++))
    test_name=$(basename "$test_file")
    exp_file="tests/${test_name}.exp"
    
    # Run the test and capture output
    # Using --debug to capture final stack states
    output=$(cargo run --package ryzelang-cli --quiet -- "$test_file" --debug 2>&1)
    
    if [ "$UPDATE_EXPECTATIONS" = true ]; then
        echo "$output" > "$exp_file"
        echo -e "${YELLOW}UPDATED${NC} $test_name"
        ((PASSED_COUNT++))
        continue
    fi
    
    if [ ! -f "$exp_file" ]; then
        echo -e "${RED}MISSING${NC} $test_name (No .exp file found. Run with --bless to create it.)"
        FAILED_TESTS+=("$test_name (missing .exp)")
        continue
    fi
    
    # Compare output with expectation
    if diff <(echo "$output") "$exp_file" > /dev/null; then
        echo -e "${GREEN}PASS${NC}    $test_name"
        ((PASSED_COUNT++))
    else
        echo -e "${RED}FAIL${NC}    $test_name"
        FAILED_TESTS+=("$test_name")
        # Show diff if requested or on failure?
        # diff <(echo "$output") "$exp_file"
    fi
done

echo -e "\n--- Test Summary ---"
echo -e "Total:  $TOTAL_COUNT"
echo -e "Passed: ${GREEN}$PASSED_COUNT${NC}"

if [ ${#FAILED_TESTS[@]} -ne 0 ]; then
    echo -e "Failed: ${RED}${#FAILED_TESTS[@]}${NC}"
    for failed in "${FAILED_TESTS[@]}"; do
        echo -e "  - $failed"
    done
    exit 1
else
    echo -e "All tests passed!"
    exit 0
fi
