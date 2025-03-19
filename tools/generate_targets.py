import random

# Configuration
server_url = "http://10.0.33.209:3000"  # Replace with your server's URL
num_keys = 10000  # Number of unique keys
total_mixed_requests = 100000  # Total requests for mixed workload
put_ratio = 0.2  # 20% writes in mixed workload
get_ratio = 0.8  # 80% reads in mixed workload

# Phase 1: Initialize all keys with a PUT request
init_requests = []
for i in range(1, num_keys + 1):
    key = f"key{i}"
    request = [
        f"POST {server_url}/store/{key}",
        "Content-Type: text/plain",
        "@./test_value.json"
    ]
    init_requests.append("\n".join(request))

# Phase 2: Mixed workload with 20% writes and 80% reads
# Calculate request counts for mixed workload
num_put = int(put_ratio * total_mixed_requests)
num_get = total_mixed_requests - num_put

# Generate POST requests for mixed workload
put_requests = []
for _ in range(num_put):
    key = f"key{random.randint(1, num_keys)}"
    request = [
        f"POST {server_url}/store/{key}",
        "Content-Type: text/plain",
        "@./test_value.json"
    ]
    put_requests.append("\n".join(request))

# Generate GET requests for mixed workload
get_requests = []
for _ in range(num_get):
    key = f"key{random.randint(1, num_keys)}"
    request = [f"GET {server_url}/get/{key}"]
    get_requests.append("\n".join(request))

# Combine and shuffle mixed workload requests
mixed_requests = put_requests + get_requests
random.shuffle(mixed_requests)

# Combine all requests (initialization followed by mixed workload)
all_requests = init_requests + mixed_requests

# Write to targets.txt
with open("targets.txt", "w", encoding="utf-8") as f:
    for request in all_requests:
        f.write(request + "\n\n")  # Two newlines between requests

print(f"Benchmark created with {num_keys} initial writes followed by {total_mixed_requests} mixed requests")
print(f"Mixed workload: {num_put} writes ({put_ratio*100}%) and {num_get} reads ({get_ratio*100}%)")