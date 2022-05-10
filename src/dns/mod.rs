use std::net::{
    UdpSocket,
    Ipv4Addr
};

pub enum OpCodeEnum{
    Query = 0,
    IQuery = 1,
    Status = 2,
}
pub enum RCodeEnum{
    NoErr = 0,
    FormatErr = 1,
    ServerErr = 2,
    NameErr = 3,
    NotImplErr = 4,
    RefusedErr = 5
}

pub struct DnsHeader {
    id: u16,
    header2: u16,
    qd_count: u16,
    an_count: u16,
    ns_count: u16,
    ar_count: u16
}

impl DnsHeader {
    pub fn create_header2(qr: bool, op_code: OpCodeEnum, aa: bool, tc: bool, rd:bool, ra:bool, r_code: RCodeEnum) -> u16{
        let mut line_2 = (qr as u16) << 15;
        line_2 += (op_code as u16) << 11;
        line_2 += (aa as u16) << 10;
        line_2 += (tc as u16) << 9;
        line_2 += (rd as u16) << 8;
        line_2 += (ra as u16) << 7;
        line_2 += r_code as u16;
        return line_2;
    }
}

pub struct DnsQuestion {
    q_type: u16,
    q_class: u16
}

pub struct DnsQuery {
    header: DnsHeader,
    question: DnsQuestion,
    name: String
}

impl DnsQuery {
    pub fn as_bytes(&self) -> Vec<u8> {
        let h_size = std::mem::size_of::<DnsHeader>();
        let q_size = std::mem::size_of::<DnsQuestion>();

        let header_bits : &[u8]= unsafe{
            std::slice::from_raw_parts(&self.header as *const DnsHeader as *const u8, h_size)
        };

        let question_bits : &[u8] = unsafe {
            std::slice::from_raw_parts(&self.question as *const DnsQuestion as *const u8, q_size)
        };

        let size = h_size + q_size + self.name.len();
        let mut packet: Vec<u8> = vec![0u8; size+1];
        packet[0..h_size].copy_from_slice(header_bits);

        let end_name = h_size + self.name.len();
        packet[h_size .. end_name].copy_from_slice(self.name.as_bytes());

        packet[end_name..size].copy_from_slice(question_bits);

        return packet;
    }
}


pub struct DnsResolver {}

impl DnsResolver {

    pub fn get_host_by_name(host: &str)
    {
        let header_ln2 = DnsHeader::create_header2(false, OpCodeEnum::Query, false, false, true, false, RCodeEnum::NoErr);
        let dns_header = DnsHeader{
            id: 0,
            header2: header_ln2,
            qd_count: 1,
            an_count: 0,
            ns_count: 0,
            ar_count: 0
        };

        let question = DnsQuestion {
            q_type: 'A' as u16,
            q_class: 1
        };

        let mut fmt :String = DnsResolver::change_dns_name(&host);
        let query = DnsQuery {
            question: question,
            header: dns_header,
            name:fmt
        };

        let packet = query.as_bytes();

        let socket = UdpSocket::bind("127.1.1.1:5353").unwrap();
        let success = socket.send_to(&packet, "172.18.64.1:53").is_ok();

        let mut buf = [0; 2048];
        socket.recv_from(&mut buf).unwrap();
        println!("{:?}", buf);
    }

    fn change_dns_name(host: &str) -> String {
        let mut formatted = String::new();

        let split = host.split(".");
        for s in split {
            formatted = format!("{}{}{}",formatted, s.len(), s);
        }
        return formatted;
    }
}