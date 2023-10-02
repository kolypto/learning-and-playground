Raspberry Pi + 4G Huawei E8382 + Wireguard
==========================================


Raspberry Pi Imager: Install Pi OS

Hostname: teslafi
SSH:
    User: pi ; password: Adm1n1str4t0r

Wifi: Tesla-Fi
Password: HowDoYouLikeIt

WiFi: HUAWEI_4B92
Password: 1306Q05124E

Install RaspberryOS
-------------------

1. Install Raspberry Pi Imager

    ```console
    $ sudo snap install rpi-imager
    ```

2. Choose Raspberry Pi OS, with Desktop
3. Install onto the SD card
4. Insert the SD card into the device
5. Use configurator: enable SSH, enable predictable interfaces

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
$ cat <<"EOF" > docker-compose.yml
version: "2.1"
services:
  wireguard:
    image: lscr.io/linuxserver/wireguard
    container_name: wireguard
    cap_add:
      - NET_ADMIN
      - SYS_MODULE
    environment:
      - PUID=1000
      - PGID=1000
      - TZ=Europe/Moscow
      - SERVERURL=temp.serwant.com.ua
      - SERVERPORT=51820
      # Peer names, or just a number to generate N peers
      - PEERS=test,teslafi,mobile
      - PEERDNS=auto
      - INTERNAL_SUBNET=10.13.13.0
      - ALLOWEDIPS=0.0.0.0/0
    volumes:
      - /home/kolypto/wireguard/config:/config
      - /lib/modules:/lib/modules
    ports:
      - 51820:51820/udp
    sysctls:
      - net.ipv4.conf.all.src_valid_mark=1
    restart: unless-stopped
EOF
$ docker-compose up
```

Copy this file from the Wireguard server:

> config/peer_teslafi/peer_teslafi.conf

Raspberry Pi Device
-------------------

Copy this file from the Wireguard server:

> config/peer_teslafi/peer_teslafi.conf

to your computer (Pi, in our case) and put the file there:

> /etc/wireguard/wg0.conf

```console
$ wg-quick up wg0
$ curl https://api.myip.com

$ sudo systemctl enable wg-quick@wg0.service
```

Now check that your VPN is up and running:

```console
$ curl https://api.myip.com/
```

4G Model: Huawei E8372
----------------------

Problem: when the device is connected, it acts as a USB stick with drivers.
We want to send it a command so that it switches to "USB modem" mode.

```console
$ sudo apt install usb-modeswitch

$ cat <<"EOF" | sudo tee '/usr/share/usb_modeswitch/12d1:1f01'
# Huawei E353 (3.se)
TargetVendor= 0x12d1
TargetProduct= 0x14db

MessageContent="55534243123456780000000000000a11062000000000000100000000000000"
#MessageContent="55534243123456780000000000000011063000000100000000000000000000"
NoDriverLoading=1
#HuaweiMode=1
EOF

$ cat <<"EOF" | sudo tee /etc/modprobe.d/huawei-noprobe.conf
options usb-storage quirks=12d1:1f01:s
EOF

$ lsusb
mode 2: smart ethernet device with webui
Bus 001 Device 004: ID 12d1:14db Huawei Technologies Co., Ltd. E353/E3131
mode 3: stupid usb modem
Bus 001 Device 004: ID 12d1:1001 Huawei Technologies Co., Ltd. E161/E169/E620/E800 HSDPA Modem
```

Most online manuals say that you need a `/dev/ttyUSB0` device.
No you don't. This modem creates a network interface: `enx001e101f0000`.
It should already work!

If not, open a browser on Pi and navigate here:

Huawei admin: http://192.168.8.1/
Password: admin

There, set up your 4G connection.
It will also become a Wi-Fi access point!


WiFi Access Point
-----------------

Now let's make our Raspberry Pi into a Wifi Access point.

Links:

* <https://thepi.io/how-to-use-your-raspberry-pi-as-a-wireless-access-point/>
* <https://blog.stigok.com/2019/03/26/raspberry-pi-wifi-ap-wireguard-port-53.html>

```console
$ sudo apt install hostapd dnsmasq

$ cat <<"EOF" | sudo tee -a /etc/dhcpcd.conf
interface wlan0
static ip_address=192.168.0.1/24
nohook wpa_supplicant
#denyinterfaces eth0
#denyinterfaces wlan0
EOF

$ cat <<"EOF" | sudo tee /etc/dnsmasq.d/wlan0.conf
interface=wlan0
except-interface=eth0
except-interface=wg0
listen-address=192.168.0.1
dhcp-range=192.168.0.100,192.168.0.200,255.255.255.0,24h
dhcp-option=option:dns-server,192.168.0.1
dhcp-authoritative
enable-ra
bogus-priv
domain-needed
EOF

$ cat <<"EOF" | sudo tee /etc/hostapd/hostapd.conf
interface=wlan0
hw_mode=g
channel=7
wmm_enabled=0
macaddr_acl=0
auth_algs=1
ignore_broadcast_ssid=0
wpa=2
wpa_key_mgmt=WPA-PSK
wpa_pairwise=TKIP
rsn_pairwise=CCMP
ssid=Tesla-Fi
wpa_passphrase=HowDoYouLikeIt
EOF

$ sudo sed -iE 's!^#DAEMON_CONF=""!DAEMON_CONF="/etc/hostapd/hostapd.conf"!' /etc/default/hostapd
$ echo 'net.ipv4.ip_forward=1' | sudo tee /etc/sysctl.d/97-wifi-ap.conf
```

We need to enable hostapd service:

```console
$ sudo systemctl unmask hostapd
$ sudo systemctl enable hostapd
$ sudo systemctl start hostapd
```

Now, we want all traffic to go through `wg0`: our VPN interface.
Normally, we use network bridges for that sort of stuff, but `wg0` is a Level 3 interface,
and it can't be bridged with a Level 2 `wlan0` interface!

So we use nftables firewall rules:

```bash
sudo nft add table nat
sudo nft 'add chain nat postrouting { type nat hook postrouting priority 100 ; }'
sudo nft add rule nat postrouting masquerade

sudo nft add table ip filter
sudo nft add table ip nat
sudo nft add chain ip filter FORWARD { type filter hook input priority 0 \; }
sudo nft add chain nat POSTROUTING { type nat hook postrouting priority 100 \; }

sudo nft add rule ip filter FORWARD iifname "wlan0" oifname "wg0" counter accept
sudo nft add rule ip filter FORWARD iifname "wg0" oifname "wlan0" ct state related,established  counter accept
sudo nft add rule ip nat POSTROUTING oifname "wg0" counter masquerade

sudo nft list ruleset
```

P.S.
----

### Bridge `wlan0` with the 4G modem
If you want to route traffic to `enx001e101f0000` instead, this is how it would look:

```console

$ sudo apt install bridge-utils
$ sudo brctl addbr br0
$ sudo brctl addif br0 enx001e101f0000

$ cat <<"EOF" | sudo tee /etc/network/interfaces.d/br0
auto br0
iface br0 inet manual
bridge_ports enx001e101f0000 wlan0
EOF
```

### Connect to a Wi-Fi
If you want to temporarily connect to a WiFi AP, switch off the WiFi, turn off the AP mode:

```console
$ sudo service hostapd stop
```

then choose a network (GUI), and do this:

```console
$ sudo dhclient wlan0
```
