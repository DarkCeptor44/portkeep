use crate::config::{PortDetailResponse, PortStatus};
use netstat2::{AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, TcpState, get_sockets_info};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use sysinfo::{Pid, ProcessesToUpdate, System};

/// Scans active TCP listening ports on the system and pairs them with your registered descriptions map
///
/// ## Arguments
///
/// * `registered_ports` - A map of port numbers to descriptions
///
/// ## Returns
///
/// * `Vec<PortDetailResponse>` - A vector of [`PortDetailResponse`] structs
pub fn scan_ports(registered_ports: &BTreeMap<u16, String>) -> Vec<PortDetailResponse> {
    let mut active_listeners: HashMap<u16, (u32, Option<String>)> = HashMap::new();

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP;

    if let Ok(sockets) = get_sockets_info(af_flags, proto_flags) {
        let mut sys = System::new_all();
        sys.refresh_processes(ProcessesToUpdate::All, true);

        for socket in sockets {
            if let ProtocolSocketInfo::Tcp(tcp_info) = socket.protocol_socket_info {
                if tcp_info.state == TcpState::Listen {
                    let port = tcp_info.local_port;

                    if let Some(&pid_u32) = socket.associated_pids.first() {
                        let proc_name = sys
                            .process(Pid::from(pid_u32 as usize))
                            .map(|proc| proc.name().to_string_lossy().into_owned());

                        active_listeners.insert(port, (pid_u32, proc_name));
                    } else {
                        active_listeners.insert(port, (0, None));
                    }
                }
            }
        }
    }

    let mut all_ports: BTreeSet<u16> = registered_ports.keys().copied().collect();
    all_ports.extend(active_listeners.keys().copied());

    all_ports
        .into_iter()
        .map(|port| {
            let description = registered_ports.get(&port).cloned();
            let active_info = active_listeners.get(&port);

            let status = match active_info {
                Some((pid, proc_name)) => PortStatus {
                    is_listening: true,
                    pid: if *pid > 0 { Some(*pid) } else { None },
                    process_name: proc_name.clone(),
                },
                None => PortStatus {
                    is_listening: false,
                    pid: None,
                    process_name: None,
                },
            };

            PortDetailResponse {
                port,
                description,
                status,
            }
        })
        .collect()
}
