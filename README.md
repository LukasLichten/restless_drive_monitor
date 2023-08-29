Intention with this project is to solve a simple concern for me:  
Monitoring the drive status of my NAS box running Proxmox with TrueNAS in a VM.

## Why?

Proxmox might still have access to the smart data, and TrueNAS has it's own alerts, but Proxmox does not output smart info through the build in Metric Server connection, and TrueNAS has a websockt and rest api, but only with auth, and tools like [Uptime Kuma](https://github.com/louislam/uptime-kuma) have limited processing, and will have then the auth key in plain text in the web portal accessible settings.  

So this application takes care of querrying and gathering this data, and allows you to just run it on your Proxmox instance, and then request the data from a simple open API.  
As such it is not secure, as anyone on your network can just see all your drives health data.

## WIP:
- ~~getting a drive list from lsblk~~
- ~~reading smart data via smartctl~~
- ~~reading truenas alerts~~
- ~~installing service~~
- reading pool status
- handle nvme
- push truenas alerts to ntfy/gotify
- add athetication


## Requirements:
- smartctl  
- systemd (for the installer, openrc is planned, you can set up your own service still)
  
Must run as super user due to requiring access to smartctl

## Usage
### Running:
```
Usage: restless_drive_monitor [OPTIONS]

Options:
  -i, --install          Installs the software in /usr/bin and creates a service to run it
  -c, --config <CONFIG>  Where the config file is located
  -h, --help             Print help
  -V, --version          Print version
```
For installation simply run
```
sudo ./restless_drive_monitor -i
```
this will copy the executable into /usr/bin and start up the service.  
This can be used to install updates (then the ```./``` is essential, as you would be else referring to the old version).  
When you don't pass any arguments the server will spin up in place.  
  
The config of the service is saved under ```/etc/restless_drive_monitor/rdm.conf```, after editing it make sure to reboot the service with
```
sudo systemctl restart restless-drive-monitor.service
```  
If you don't pass any arguments, then a rdm.conf will be created/used in the current folder.

### Reading:
Follwing functions are available:
```
/ping
/drivelist
/smart/[drive]
/smart/disk/by-id/[drive]
/alerts
/alerts/[level]
```
```src/data.rs``` contains the datamodel for the api. If you want to use a rust aplication you can copy this file into your project and you have functioning parsing immediatly.

## Build
### Requirements:
Rustc 1.70 or higher

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