# mainline-dht-gcp-ping-test

This repository provides the results of a ping test to a list of IP:PORT addresses of [Mainline DHT nodes](https://en.wikipedia.org/wiki/Mainline_DHT). The ping is a simple UDP packet sent to a node that should return a UDP pong. This way, it can measure how well a server is connected to the wider Mainline DHT.

[mainline-dht-ping.sh](./mainline-dht-ping.sh) is the script to [ping](https://www.bittorrent.org/beps/bep_0005.html#ping) nodes in the DHT. It uses netcat to send a simple UDP ping packet.

```bash
./mainline-dht-ping.sh test-03-7-2025/ip-list.txt
```

A [list](./test-03-7-2025/ip-list.txt) of 838 IP:PORT combinations of DHT nodes has been made at the 3rd of July 2025 to conduct this test.

## Cloud Provider Results 3rd of July 2025

[**Google Cloud**](./test-03-7-2025/gcp-results.txt) 0.35% success rate

```
Mainline DHT Ping
Read 838 socket addresses
Ping:
- 2.123.167.163:6882 - No response
- 47.149.8.188:49049 - No response
...
- 223.122.155.11:49064 - No response
- 222.241.136.162:44903 - No response

3 out of 838 socket addresses responded successfully.
Success rate: 0.35%
```

[**Hetzner**](./test-03-7-2025/hetzner-results.txt) 87.82% success rate

```
Mainline DHT Ping
Read 838 socket addresses
Ping:
- 2.123.167.163:6882 - No response
- 47.149.8.188:49049 - No response
...
- 222.116.47.59:32806 - Pong success
- 189.28.91.132:21254 - Pong success

736 out of 838 socket addresses responded successfully.
Success rate: 87.82%
```

[**Digital Ocean**](./test-03-7-2025/digitalocean-results.txt) 84.62% success rate

```
Mainline DHT Ping
Read 839 socket addresses
Ping:
- 37.78.65.185:6881 - No response
- 45.167.83.209:53071 - No response
...
- 222.116.47.59:32806 - Pong success
- 222.241.136.162:44903 - Pong success

710 out of 839 socket addresses responded successfully.
Success rate: 84.62%
```

[**Residential Switzerland**](./test-03-7-2025/localhost-results.txt) 86.63% success rate

```
Mainline DHT Ping
Read 838 socket addresses
Ping:
- 2.123.167.163:6882 - No response
- 47.149.8.188:49049 - No response
...
- 221.163.162.80:33001 - Pong success
- 222.120.148.176:33113 - Pong success

726 out of 838 socket addresses responded successfully.
Success rate: 86.63%
```

[**Amazon Web Services**](./test-03-7-2025/aws-results.txt) 0.83% success rate

```
Mainline DHT Ping
Read 839 socket addresses
Ping:
- 2.123.167.163:6882 - No response
- 37.78.65.185:6881 - No response
...
- 223.122.155.11:49064 - No response
- 222.241.136.162:44903 - No response

7 out of 839 socket addresses responded successfully.
Success rate: 0.83%
```


## IP List

The [provided IP list](./test-03-7-2025/ip-list.txt) was created with the help of [pubky/mainline](https://github.com/pubky/mainline) on the 3rd of July 2025. Individual DHT nodes go offline ocassionally and the provided IP list may need to be refreshed after a while.

Nonetheless, the stark success rate contrast between GCP (0.35%) and the other listed cloud services (~85%) is immense.