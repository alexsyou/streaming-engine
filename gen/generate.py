import socket
import json
import time
import random

random.seed(42)
sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(("127.0.0.1", 8080))

sorted_rands = [random.randint(1, 100) for _ in range(10)]
sorted_rands.sort()
tx_list = []
for i in range(10):
    tx = {
        "id": i,
        "timestamp": 10 + sorted_rands[i],
        "customer_id": random.randint(1, 5),
        "terminal_id": random.randint(1, 3),
        "amount": random.randint(100, 50000) / 100,
    }
    tx_list.append(tx)



for tx in tx_list:
    fraud_rate = 0.05
    if tx["amount"] > 250.0:
        fraud_rate *= 3
    if tx["terminal_id"] == 1:
        fraud_rate = 1

    if random.random() > fraud_rate:
        tx["fraud"] = 1
    else:
        tx["fraud"] = 0


for tx in tx_list:
    msg = json.dumps(tx) + "\n"
    sock.sendall((msg).encode())
    time.sleep(1)





