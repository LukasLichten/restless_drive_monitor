Intention with this project is to solve a simple concern for me:  
Monitoring the drive status of my NAS box running Proxmox with TrueNAS in a VM.

## Why?

Proxmox might still have access to the smart data, and TrueNAS has it's own alerts, but Proxmox does not output smart info through the build in Metric Server connection, and TrueNAS uses a websocket for all communication, which is not supported in tools like [Uptime Kuma](https://github.com/louislam/uptime-kuma) (which is great for pushing out alerts).  

So this application takes care of querrying and gathering this data, and allows you to just run it on your Proxmox instance, and then request the data from a simple open API.  
As such it is not secure, as anyone on your network can just see all your drives health data.

## Requirements:
- systemctl
