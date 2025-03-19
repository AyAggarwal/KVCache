import pandas as pd
import matplotlib.pyplot as plt

# Load the CSV files
lockfree_data = pd.read_csv("../results/LOCKFREE.csv")
lockfree_h2c_data = pd.read_csv("../results/LOCKFREE-H2C.csv")

# Plot mean latency comparison
plt.figure(figsize=(10, 6))
plt.plot(lockfree_data["rate"], lockfree_data["latency_mean"], label="LOCKFREE Mean Latency", marker="o")
plt.plot(lockfree_h2c_data["rate"], lockfree_h2c_data["latency_mean"], label="LOCKFREE-H2C Mean Latency", marker="x")
plt.xlabel("Request Rate (req/s)")
plt.ylabel("Latency (ms)")
plt.title("Mean Latency Comparison: LOCKFREE vs LOCKFREE-H2C")
plt.legend()
plt.grid(True)
plt.savefig("../results/lockfree_comparison.png")
plt.show()
