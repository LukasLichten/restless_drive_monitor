Intention with this project is to solve a simple concern for me:  
Monitoring the drive status of my NAS box running Proxmox with TrueNAS in a VM.

## Why?

Proxmox might still have access to the smart data, and TrueNAS has it's own alerts, but Proxmox does not output smart info through the build in Metric Server connection, and TrueNAS has a websockt and rest api, but only with auth, and tools like [Uptime Kuma](https://github.com/louislam/uptime-kuma) (which is great for pushing out alerts) don't easily support querrying from it (you can, but the token will be in plain text in the settings of the monitor, and processing the data is pretty limited).  

So this application takes care of querrying and gathering this data, and allows you to just run it on your Proxmox instance, and then request the data from a simple open API.  
As such it is not secure, as anyone on your network can just see all your drives health data.

## WIP
- ~~getting a drive list from lsblk~~
- ~~reading smart data via smartctl~~
- ~~reading truenas alerts~~
- reading pool status
- handle nvme
- push truenas alerts to ntfy/gotify
- add athetication
- installing service

## Requirements:
- smartctl  
  
Must run as super user due to requiring access to smartctl

## Build
### Debug:  
```
cargo build
sudo ./target/debug/restless_drive_monitor
```
or use dbg.sh  
This is because we can't use cargo run due to requiring sudo  
You could debug however all the none smartctl functions still
  
### Release:  
```
cargo build --release
```