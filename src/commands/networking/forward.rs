use std::{fmt::Display, process::Command};

use crate::{ADBCommand, ADBResult};

/// List all forward socket connections
pub struct ADBForward {
    shell: Command,
}

pub enum ForwardMethodLocal {
    /// May be 0 to pick any open port
    Tcp(u32),
    LocalAbstract(UDS),
    LocalReserved(UDS),
    LocalFilesystem(UDS),
}

pub enum ForwardMethodRemote {
    Tcp(u32),
    // LocalAbstract(UDS),
    // LocalReserved(UDS),
    // LocalFilesystem(UDS),
    /// PID
    Jdwp(u32),
    /// CID, PID
    Vsock(u32, u32),
}

impl ADBForward {
    /// List all forward socket connections
    pub fn list() -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg("forward").arg("--list");

        ADBForward { shell: cmd }
    }

    /// Forward socket connection using a local and remote method
    pub fn using(local: ForwardMethodLocal, remote: ForwardMethodRemote) -> Self {
        let mut cmd = Command::new("adb");
        cmd.arg(match local {
            ForwardMethodLocal::Tcp(port) => format!("tcp:{port}"),
            ForwardMethodLocal::LocalAbstract(uds) => format!("localabstract:{uds}"),
            ForwardMethodLocal::LocalReserved(uds) => format!("localreserved:{uds}"),
            ForwardMethodLocal::LocalFilesystem(uds) => format!("localfilesystem:{uds}"),
        });
        cmd.arg(match remote {
            ForwardMethodRemote::Tcp(port) => format!("tcp:{port}"),
            // ForwardMethodRemote::LocalAbstract(_) => todo!(),
            // ForwardMethodRemote::LocalReserved(_) => todo!(),
            // ForwardMethodRemote::LocalFilesystem(_) => todo!(),
            ForwardMethodRemote::Jdwp(pid) => format!("jdwp:{pid}"),
            ForwardMethodRemote::Vsock(cid, pid) => format!("vsock:{cid}:{pid}"),
        });

        ADBForward { shell: cmd }
    }
}

impl ADBCommand for ADBForward {
    fn build(&mut self) -> Result<&mut Command, String> {
        Ok(&mut self.shell)
    }

    fn process_output(&self, output: ADBResult) -> ADBResult {
        output
    }
}

/// Unix domain socket or IPC (https://en.wikipedia.org/wiki/Unix_domain_socket)
pub enum UDS {
    /// stream oriented
    SockStream,
    // datagram oriented, preserves message boundaries
    SockDgram,
    // connection oriented, sequenced-packet
    SockSeqpacket,
}

impl Display for UDS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UDS::SockStream => "SOCK_STREAM",
                UDS::SockDgram => "SOCK_DGRAM",
                UDS::SockSeqpacket => "SOCK_SEQPACKET",
            }
        )
    }
}
