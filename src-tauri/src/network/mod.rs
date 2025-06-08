use mdns_sd::{ServiceDaemon, ServiceInfo};
use mdns_sd::ServiceEvent;
use std::sync::mpsc::Sender;
use std::net::{IpAddr, Ipv4Addr};
use tauri::async_runtime::spawn;
use futures_util::future::pending;

const SERVICE_TYPE: &str = "_openwboard._tcp.local.";
const INSTANCE_NAME: &str = "openwboard-instance";
const SERVICE_PORT: u16 = 42424;

pub fn register_service() {
    spawn(async move {
        let daemon = ServiceDaemon::new().expect("Failed to start mDNS daemon");

        let my_service = ServiceInfo::new(
            "_openwboard._tcp.local.",
            INSTANCE_NAME,
            "openwboard.local.",
            &[IpAddr::V4(Ipv4Addr::LOCALHOST)][..],
            0,
            None,
        ).expect("Failed to create service info");

        daemon.register(my_service).expect("Failed to register mDNS service");

        // Keep the service alive
        pending::<()>().await;
    });
}


pub fn discover_peers(found: Sender<String>) {
    spawn(async move {
        let daemon = ServiceDaemon::new().expect("Failed to start mDNS daemon");
        let receiver = daemon.browse(SERVICE_TYPE).expect("Failed to browse service");

        for event in receiver {
            if let ServiceEvent::ServiceResolved(info) = event {
                if let Some(addr) = info.get_addresses().iter().next() {
                    let peer = format!("{}:{}", addr, info.get_port());
                    let _ = found.send(peer);
                }
            }
        }
    });
}
