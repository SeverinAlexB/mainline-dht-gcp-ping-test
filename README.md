# mainline-dht-gcp-ping-test

This repository provides the results of a ping test to a list of IP:PORT addresses of [Mainline DHT nodes](https://en.wikipedia.org/wiki/Mainline_DHT). The ping is a simple UDP packet sent to a node that should return a UDP pong. This way, it can measure how well a server is connected to the wider Mainline DHT.

[mainline-dht-ping.sh](./mainline-dht-ping.sh) is the main script to [ping](https://www.bittorrent.org/beps/bep_0005.html#ping) nodes in the mainline DHT.

```bash
$ ./mainline-dht-ping.sh mainline-ips-03-07-2025.txt

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

## Cloud Provider Results 3rd of July 2025

[**Google Cloud**](./gcp-results-03-07-2025.txt) 0.35% success rate

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

[**Hetzner**](./hetzner-results-03-07-2025.txt) 87.82% success rate

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

[**Digital Ocean**](./digitalocean-03-07-2025.txt) 84.62% success rate

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

[**Residential Switzerland**](./localhost-results-03-07-2025.txt) 86.63% success rate

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
