import socket
import json
import time
import random

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.connect(("127.0.0.1", 8080))

i = 0
timestamp = int(time.time())

while True:
    tx = {
        "id": i,
        "timestamp": timestamp,
        "customer_id": random.randint(1, 5),
        "terminal_id": random.randint(1, 3),
        "amount": random.randint(100, 50000) / 100,
        "fraud": 0,
    }

    msg = json.dumps(tx) + "\n"
    sock.sendall(msg.encode())
    print(f"Sent: {msg.strip()}")

    i += 1
    timestamp += random.randint(1, 10)
    time.sleep(1)
