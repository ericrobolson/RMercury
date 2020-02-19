pub mod ringbuffer;

pub const MAX_UDP_ENDPOINTS: u8 = 16;
pub const MAX_UDP_PACKET_SIZE: u16 = 4096;

pub struct UdpStats {
    pub bytes_sent: i32,
    pub packets_sent: i32,
    pub kbps_sent: i32,
}

pub trait UdpChannel {
    fn OnMessage();
}

pub struct Udp {
    pub stats: UdpStats,
}
