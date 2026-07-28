#!/usr/bin/python
__author__ = "ma11ock"

import threading, socket, time

tcpSocket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
# allow address to be re-used
tcpSocket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
tcpSocket.bind(("0.0.0.0", 8000))
tcpSocket.listen(2)
print("Waiting for a Client ..")


def echo_server(id):
    client, (ip, port) = tcpSocket.accept()

    print("rcvd connection from : ", ip)
    print("starting echo output ...")
    time.sleep(2)
    data = "dummy"

    while len(data):
        data = client.recv(2048)
        print(threading.current_thread())
        print("ID = %d " % id)
        print(ip, " sent: ", data)
        client.send(data)

    client.close()


threads = list()

for i in range(2):
    thread = threading.Thread(target=echo_server, args=(i,))
    thread.start()
    time.sleep(2)
    threads.append(thread)
