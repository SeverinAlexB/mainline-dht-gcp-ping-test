#!/bin/bash

if [ $# -ne 1 ]; then
  echo "Usage: $0 <ip_list_file>"
  exit 1
fi

IP_FILE="$1"

if [ ! -f "$IP_FILE" ]; then
  echo "Error: File $IP_FILE not found"
  exit 1
fi

PACKET="d1:ad2:id20:abcdefghij0123456789e1:q4:ping1:t2:aa1:y1:qe"

# Count only non-empty lines
total=0
success_count=0

# Count actual addresses first
while IFS= read -r line; do
  [ -n "$line" ] && ((total++))
done <"$IP_FILE"

echo "Mainline DHT Ping"
echo "Read $total socket addresses"
echo "Ping:"

# Function to ping a single address
ping_address() {
  local ip_port="$1"
  local ip=$(echo "$ip_port" | cut -d':' -f1)
  local port=$(echo "$ip_port" | cut -d':' -f2)

  response=$(echo -n "$PACKET" | nc -u -w2 "$ip" "$port" 2>/dev/null)

  if [ -n "$response" ]; then
    echo "- $ip:$port - Pong success"
    echo "SUCCESS" >>/tmp/dht_results.$$
  else
    echo "- $ip:$port - No response"
  fi
}

# Clean up temp file
rm -f /tmp/dht_results.$$

# Launch all pings in parallel
while IFS= read -r line; do
  [ -z "$line" ] && continue
  ping_address "$line" &
done <"$IP_FILE"

# Wait for all background jobs to complete
wait

# Count successes and trim whitespace
if [ -f /tmp/dht_results.$$ ]; then
  success_count=$(wc -l </tmp/dht_results.$$ | tr -d ' ')
  rm -f /tmp/dht_results.$$
fi

# Calculate success rate with 2 decimals using integer arithmetic
# Multiply by 10000 to get 2 decimal places, then format
rate_calc=$((success_count * 10000 / total))
whole_part=$((rate_calc / 100))
decimal_part=$((rate_calc % 100))

# Format decimal part with leading zero if needed
if [ $decimal_part -lt 10 ]; then
  success_rate="${whole_part}.0${decimal_part}"
else
  success_rate="${whole_part}.${decimal_part}"
fi

echo
echo "$success_count out of $total socket addresses responded successfully."
echo "Success rate: $success_rate%"
