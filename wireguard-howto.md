
Install WireGuard on Your Server
--------------------------------

This server will accept incoming connections and act as your VPN server:

Links:

* <https://github.com/linuxserver/docker-wireguard>
* <https://habr.com/ru/post/486452/>

```
$ sudo apt install docker.io
$ mkdir -p ~/wireguard/config
$ cd ~/wireguard/
$ cat <<EOF > docker-compose.yml
version: "3.8"
services:
  wireguard:
    image: linuxserver/wireguard
    container_name: wireguard
    cap_add:
      - NET_ADMIN
      - SYS_MODULE
    environment:
      # Set your server hostname. It must be accessible from the Internet
      - SERVERURL=78.153.130.167
      - SERVERPORT=51820
      # List: names of users to generate accounts for
      # Alternative: write '10' to generate 10 accounts without names
      - PEERS=10
      - PEERDNS=auto
      # Process and network
      - PUID=${UID}
      - PGID=${UID}
      - TZ=Europe/Moscow
      - INTERNAL_SUBNET=10.13.13.0
      - ALLOWEDIPS=0.0.0.0/0
    volumes:
      # Replaces $USER with your home folder
      - /home/${USER}/wireguard/config:/config
      - /lib/modules:/lib/modules
    ports:
      - 51820:51820/udp
    sysctls:
      - net.ipv4.conf.all.src_valid_mark=1
    restart: unless-stopped
EOF
$ docker-compose up
```

Linux Client Setup
------------------

Copy this file from the Wireguard server:

> config/peer_username/peer_username.conf

to your computer (Pi, in our case) and put the file there:

> /etc/wireguard/wg0.conf

```console
$ wg-quick up wg0
$ curl https://api.myip.com
```

Enable auto-up:

```console
$ sudo systemctl enable wg-quick@wg0.service
```

After restart, check that the VPN is up and running:

```console
$ curl https://api.myip.com/
```
