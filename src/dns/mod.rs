use std::net::{
    UdpSocket,
    Ipv4Addr
};

pub enum OpCodeEnum{
    Query = 0,
    IQuery = 1,
    Status = 2,
}
impl OpCodeEnum {
    pub fn from_int(val: u16) ->OpCodeEnum {
        match val {
            0 => OpCodeEnum::Query,
            1 => OpCodeEnum::IQuery,
            _ => OpCodeEnum::Status 
        }
    } 
}

pub enum RCodeEnum{
    NoErr = 0,
    FormatErr = 1,
    ServerErr = 2,
    NameErr = 3,
    NotImplErr = 4,
    RefusedErr = 5
}
impl RCodeEnum {
    pub fn from_int(val: u16) ->RCodeEnum {
        match val {
            0 => RCodeEnum::NoErr, 
            1 => RCodeEnum::FormatErr, 
            2 => RCodeEnum::ServerErr, 
            3 => RCodeEnum::NameErr, 
            4 => RCodeEnum::NotImplErr, 
            _ => RCodeEnum::RefusedErr
        }
    } 
}

struct Header2 {
    qr: bool,
    op_code: OpCodeEnum,
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    r_code: RCodeEnum
}

impl Header2 {
    pub fn to_int(self) -> u16{
        let mut line_2 = (self.qr as u16) << 15;
        line_2 += (self.op_code as u16) << 11;
        line_2 += (self.aa as u16) << 10;
        line_2 += (self.tc as u16) << 9;
        line_2 += (self.rd as u16) << 8;
        line_2 += (self.ra as u16) << 7;
        line_2 += self.r_code as u16;
        return line_2;
    }
    pub fn from_int(data: u16) -> Header2 {
        return Header2 {
            qr: (data >> 15) == 1, 
            op_code: OpCodeEnum::from_int((data & 0x7800)>>11),
            aa: (data & 0x400) == 0x400,
            tc: (data & 0x200) == 0x200,
            rd: (data & 0x100) == 0x100,
            ra: (data & 0x80) == 0x80,
            r_code: RCodeEnum::from_int(data & 0xF) 
        }
    }
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

        let size = h_size + q_size + self.name.len() + 1;
        let mut packet: Vec<u8> = vec![0u8; size+1];
        packet[0..h_size].copy_from_slice(header_bits);

        let end_name = h_size + self.name.len();
        let name_bytes = self.name.as_bytes();
        packet[h_size .. end_name].copy_from_slice(name_bytes);
        packet[end_name] = 0;
        packet[end_name+1..size].copy_from_slice(question_bits);

        return packet;
    }

}

pub struct DnsResponse {
    header: DnsHeader,
}
impl DnsResponse {
    fn byte_to_u16(bytes: &[u8], index: usize) -> u16 {
        return (bytes[index+1] as u16) << 8 | bytes[index] as u16;
    }

    pub fn from_bytes(bytes: &[u8]) -> DnsHeader{
        let header = DnsHeader {
            id: DnsResponse::byte_to_u16(&bytes, 0) ,
            header2: DnsResponse::byte_to_u16(&bytes, 2),
            qd_count: DnsResponse::byte_to_u16(&bytes, 4),
            an_count: DnsResponse::byte_to_u16(&bytes, 6),
            ns_count: DnsResponse::byte_to_u16(&bytes, 8),
            ar_count: DnsResponse::byte_to_u16(&bytes, 10)
        };
        return header;
    }
}

pub struct DnsResolver {}

impl DnsResolver {

    pub fn get_host_by_name(host: &str)
    {
        let header_ln2 = Header2{
            qr: false,
            op_code: OpCodeEnum::Query,
            aa: false,
            tc: false,
            rd: true, 
            ra: false,
            r_code:RCodeEnum::NoErr
        };

        let dns_header = DnsHeader{
            id: 1337,
            header2: header_ln2.to_int(),
            qd_count: 1,
            an_count: 0,
            ns_count: 0,
            ar_count: 0
        };

        let question = DnsQuestion {
            q_type:  1,
            q_class: 1
        };

        let mut fmt :String = DnsResolver::change_dns_name(&host);
        let query = DnsQuery {
            question: question,
            header: dns_header,
            name: String::from(host)
        };

        let packet = query.as_bytes();

        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        let success = socket.send_to(packet.as_ref(), "8.8.8.8:53").is_ok();

        let mut buf = [0; 2048];
        let len = socket.recv_from(&mut buf).unwrap();
        let header = DnsResponse::from_bytes(&buf);
        let h2 = Header2::from_int(header.header2);
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