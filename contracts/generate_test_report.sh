#!/bin/bash

echo "Running tests and generating HTML report..."

HTML_FILE="results.html"

MAX_RETRIES=15
RETRY_DELAY=6 # seconds

TEST_LIST_FILE="test_list.tmp"
FILTERED_FILE="filtered_test_list.tmp"
DEBUG_FILE="debug.tmp"
FINAL_FILTERED_FILE="final_filtered_test_list.tmp"

declare -a contracts

# Clear the HTML file and temporary files
true >"$HTML_FILE"
true >"$TEST_LIST_FILE"
true >"$FILTERED_FILE"
true >"$DEBUG_FILE"
true >"$FINAL_FILTERED_FILE"

cargo test -- --list >"$TEST_LIST_FILE" 2>&1

if [ ! -s "$TEST_LIST_FILE" ]; then
    echo "No tests found. Exiting..."
    exit 0
fi

function contains_sc_name() {
    local prefix=$1
    for existing_contract in "${contracts[@]}"; do
        if [ "$existing_contract" = "$prefix" ]; then
            return 0
        fi
    done
    return 1
}

# Function to get the status from the API with retry logic
function get_status_with_retries() {
    local tx_hash=$1
    local retries=0
    local response
    local status

    while [ $retries -lt $MAX_RETRIES ]; do
        response=$(curl -s "https://devnet-api.multiversx.com/transactions/$tx_hash")
        status=$(echo "$response" | jq -r '.status')
        function=$(echo "$response" | jq -r '.function // .action.name') # Weird case when function is not available yet but becomes available later
        if [ "$status" != "pending" ] && [ -n "$function" ]; then
            echo "$response"
            return 0
        fi

        retries=$((retries + 1))
        sleep "$RETRY_DELAY"
    done

    echo "$response"
    return 1
}

# Process the test list file
while IFS= read -r line; do
    line="${line//[[:space:]]/}"
    if [[ "$line" =~ ^Runningtests || "$line" =~ ^Runningunittests ]]; then
        filename="${line##*/}"            # Extract the filename from the line
        pretty_filename="${filename%%-*}" # Extract a prettier version of filename from the line

        tests_found=false

        TEMP_TEST_FILE=$(mktemp)

        # Loop to read subsequent lines
        while IFS= read -r next_line; do
            if [[ "$next_line" =~ ^[[:space:]]*([a-zA-Z0-9_]+):[[:space:]]*test$ ]]; then
                echo "$next_line" >>"$TEMP_TEST_FILE"
                tests_found=true
            else
                echo "" >>"$TEMP_TEST_FILE"
                break
            fi
        done

        if [ "$tests_found" = true ]; then
            # Extract the contract name from the filename
            if [[ "$pretty_filename" == *scenario* ]]; then
                prefix="${pretty_filename%%_scenario*}"

                if ! contains_sc_name "$prefix"; then
                    contracts+=("$prefix")
                fi
            elif [[ "$pretty_filename" == *blackbox* ]]; then
                prefix="${pretty_filename%%_blackbox*}"

                if ! contains_sc_name "$prefix"; then
                    contracts+=("$prefix")
                fi
            fi
            echo "$pretty_filename" >>"$FILTERED_FILE"
            cat "$TEMP_TEST_FILE" >>"$FILTERED_FILE"
        fi

        rm "$TEMP_TEST_FILE"
    fi
done <"$TEST_LIST_FILE"

# Properly order the tests based on the contract name
for contract in "${contracts[@]}"; do
    echo "$contract" >>"$FINAL_FILTERED_FILE"

    while IFS= read -r line; do
        line="${line//[[:space:]]/}"
        if [[ "$line" == *$contract* && -n "$line" && ! "$line" =~ :test ]]; then
            echo "$line" >>"$FINAL_FILTERED_FILE"
            # Loop to read subsequent lines
            while IFS= read -r next_line; do
                if [[ -n "$next_line" ]]; then
                    echo "$next_line" >>"$FINAL_FILTERED_FILE"
                else
                    echo "" >>"$FINAL_FILTERED_FILE"
                    break
                fi
            done
        fi
    done <"$FILTERED_FILE"
done

cat <<EOF >"$HTML_FILE"
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<title>Test Results</title>
<!-- Bootstrap CSS -->
<link href="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css" rel="stylesheet">
<style>
.collapsible {
  background-color: #8B5CF6;
  color: white;
  cursor: pointer;
  padding: 10px;
  width: 100%;
  border: none;
  text-align: left;
  outline: none;
  font-size: 16px;
  font-weight: bold;
  border-radius: 4px;
  margin-bottom: 10px;
}
.content {
  padding: 6px 10px;
  display: none;
  background-color: #f1f1f1;
}
.function {
  font-weight: bold;
}
.status-success {
  color: green;
}
.status-fail {
  color: red;
}
.status-pending {
  color: orange;
}
</style>
</head>
<body class="container">
<h2 class="my-4">Test Results</h2>
EOF

current_file=""
is_interactor_file=false
current_contract=""
count_success=0
count_fail=0
count_pending=0

while IFS= read -r line; do
    line="${line//[[:space:]]/}"
    if contains_sc_name "$line"; then
        if [ -n "$current_contract" ] && [ "$current_contract" != "$line" ]; then
            if [ -n "$current_file" ]; then
                echo "</div>" >>"$HTML_FILE"
            fi
            echo "</div>" >>"$HTML_FILE"
        fi

        echo "<button class=\"collapsible\">${line}</button>" >>"$HTML_FILE"
        echo "<div class=\"content\">" >>"$HTML_FILE"
        current_contract="$line"
        current_file=""
    elif [[ ! "$line" =~ :test ]] && [[ -n "$line" ]]; then

        # Close the div for the previous file
        if [ -n "$current_file" ]; then
            echo "</div>" >>"$HTML_FILE"
        fi

        echo "<button class=\"collapsible\">${line}</button>" >>"$HTML_FILE"
        echo "<div class=\"content\">" >>"$HTML_FILE"

        current_file="$line"

        if [[ "$line" =~ .*interact.* ]]; then
            is_interactor_file=true
        else
            is_interactor_file=false
        fi

    # The line contains a test name
    elif [[ -n "$line" ]]; then
        test_name="${line%%:*}"

        # For non-interactor files, add test results under the file section
        if [ "$is_interactor_file" = false ]; then
            TEST_OUTPUT_FILE="output_${test_name}.tmp"
            cargo test "$test_name" -- --nocapture >"$TEST_OUTPUT_FILE" 2>&1

            if grep -q "ok" "$TEST_OUTPUT_FILE"; then
                status_class="status-success"
                status="Test Passed"
                count_success=$((count_success + 1))
            else
                status_class="status-fail"
                status="Test Failed"
                count_fail=$((count_fail + 1))
            fi

            echo "<p><span class=\"function\">$test_name: </span><span class=\"$status_class\">$status</span></p>" >>"$HTML_FILE"

            rm "$TEST_OUTPUT_FILE"

        # For interactor files, handle each test individually as before
        else
            echo "<button class=\"collapsible\">$test_name</button>" >>"$HTML_FILE"
            echo "<div class=\"content\">" >>"$HTML_FILE"

            TEST_OUTPUT_FILE_INTERACT="interact_output_${test_name}.tmp"
            cargo test "$test_name" -- --nocapture >"$TEST_OUTPUT_FILE_INTERACT" 2>&1

            # Extract transaction hashes from the test output
            tx_hashes=$(grep -oP '(?<=tx hash: )\S+' "$TEST_OUTPUT_FILE_INTERACT")

            if [ -z "$tx_hashes" ]; then
                echo "<p><b>No transaction hashes found for this test.</b></p>" >>"$HTML_FILE"
            else
                # For each transaction hash, make the API call and extract function and status
                while IFS= read -r tx_hash; do

                    response=$(get_status_with_retries "$tx_hash")
                    function=$(echo "$response" | jq -r '.function // .action.name') # weird case when function is not available yet but becomes available later
                    status=$(echo "$response" | jq -r '.status')
                    echo "$response" >>"$DEBUG_FILE"
                    echo "" >>"$DEBUG_FILE"
                    tx_url="https://devnet-explorer.multiversx.com/transactions/$tx_hash"
                    debug_url="https://devnet-api.multiversx.com/transactions/$tx_hash"

                    # Determine the status class
                    case "$status" in
                    "success")
                        status_class="status-success"
                        echo "<p><span class=\"function\">Function: $function</span> | <span class=\"$status_class\">Status: $status</span> <a href=\"$debug_url\" target=\"_blank\">See on explorer</a> </p>" >>"$HTML_FILE"
                        count_success=$((count_success + 1))
                        ;;
                    "fail")
                        status_class="status-fail"
                        error=$(echo "$response" | jq -r '.operations[0].message // "No error available"')
                        echo "<p><span class=\"function\">Function: $function</span> | <span class=\"$status_class\">Status: $status (Error: <a href=\"$tx_url\" target=\"_blank\">$error</a>)</span></p>" >>"$HTML_FILE"
                        count_fail=$((count_fail + 1))
                        ;;
                    "pending")
                        status_class="status-pending"
                        echo "<p><span class=\"function\">Function: $function</span> | <span class=\"$status_class\">Status: $status (<a href=\"$tx_url\" target=\"_blank\">See on explorer</a>)</span></p>" >>"$HTML_FILE"
                        count_pending=$((count_pending + 1))
                        ;;
                    *)
                        status_class="status-pending"
                        echo "<p><span class=\"function\">Function: $function</span> | <span class=\"$status_class\">Status: $status (<a href=\"$tx_url\" target=\"_blank\">See on explorer</a>)</span></p>" >>"$HTML_FILE"
                        count_pending=$((count_pending + 1))
                        ;;
                    esac

                done <<<"$tx_hashes"
            fi

            echo "</div>" >>"$HTML_FILE"
            rm "$TEST_OUTPUT_FILE_INTERACT"
        fi
    fi

done <"$FINAL_FILTERED_FILE"

echo "</div>" >>"$HTML_FILE"
echo "</div>" >>"$HTML_FILE"

total_tests=$((count_success + count_fail + count_pending))

echo -e "<h3 class=\"mt-4\">Summary</h3>\n<p><b>Total tests: $total_tests  </b><b>Success: </b><span class=\"status-success\">$count_success</span>  <b>Fail: </b><span class=\"status-fail\">$count_fail</span>  <b>Pending: </b><span class=\"status-pending\">$count_pending</span></p>" >>"$HTML_FILE"

cat <<EOF >>"$HTML_FILE"
<script src="https://code.jquery.com/jquery-3.5.1.slim.min.js"></script>
<script src="https://cdn.jsdelivr.net/npm/@popperjs/core@2.5.4/dist/umd/popper.min.js"></script>
<script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/js/bootstrap.min.js"></script>
<script>
var coll = document.getElementsByClassName("collapsible");
var i;
for (i = 0; i < coll.length; i++) {
  coll[i].addEventListener("click", function() {
    this.classList.toggle("active");
    var content = this.nextElementSibling;
    if (content.style.display === "block") {
      content.style.display = "none";
    } else {
      content.style.display = "block";
    }
  });
}
</script>
</body>
</html>
EOF

rm "$TEST_LIST_FILE" "$FILTERED_FILE" "$FINAL_FILTERED_FILE" "$DEBUG_FILE"

echo "HTML report generated: $HTML_FILE"
