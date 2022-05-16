use std::{net::{
    UdpSocket,
    Ipv4Addr
}, io::Read, borrow::Borrow, convert::TryInto};

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
    pub fn new(id:u16, h2: u16, qd: u16, an: u16, ns: u16, ar: u16) -> DnsHeader{
        DnsHeader { id: id.to_be(), header2: h2, qd_count: qd.to_be(), an_count: an.to_be(), ns_count: ns.to_be(), ar_count: ar.to_be() }
    }
}

pub struct DnsQuestion {
    q_type: u16,
    q_class: u16
}
impl DnsQuestion {
    pub fn new(qt: u16, qc: u16) -> DnsQuestion {
        DnsQuestion { q_type: qt.to_be(), q_class: qc.to_be() }
    } 
}

pub struct DnsQuery {
    header: DnsHeader,
    question: DnsQuestion,
    name: String
}

impl DnsQuery {
    fn change_dns_name(host:&String) -> Vec<u8>{
        let mut formatted: Vec<u8> = Vec::new();

        let split = host.split(".");
        for s in split {
            formatted.push(s.len() as u8);
            for c in s.bytes() {
                formatted.push(c as u8);
            }
        }
        formatted.push(0);
        return formatted;
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let h_size = std::mem::size_of::<DnsHeader>();
        let q_size = std::mem::size_of::<DnsQuestion>();

        let header_bits : &[u8]= unsafe{
            std::slice::from_raw_parts(&self.header as *const DnsHeader as *const u8, h_size)
        };

        let question_bits : &[u8] = unsafe {
            std::slice::from_raw_parts(&self.question as *const DnsQuestion as *const u8, q_size)
        };

        let name_bytes: Vec<u8> = DnsQuery::change_dns_name(&self.name);
        let end_name = h_size + name_bytes.len();
        let size = end_name + q_size;

        let mut packet: Vec<u8> = vec![0u8; size];
        packet[0..h_size].copy_from_slice(&header_bits);
        packet[h_size .. end_name].copy_from_slice(&name_bytes);
        packet[end_name..size].copy_from_slice(question_bits);

        return packet;
    }

}

pub struct DnsResponse {
    header: DnsHeader,
    name: String,
    question: DnsQuestion ,
    answer: DnsQuestion,
    ttl: u32,
    address: String
}

impl DnsResponse {
    pub fn from_bytes(bytes: &[u8], len: usize) -> DnsResponse{
        let header = DnsHeader {
            id: u16::from_be_bytes(bytes[0..2].try_into().unwrap()),
            header2: u16::from_be_bytes(bytes[2..4].try_into().unwrap()), 
            qd_count: u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
            an_count: u16::from_be_bytes(bytes[6..8].try_into().unwrap()),
            ns_count: u16::from_be_bytes(bytes[8..10].try_into().unwrap()),
            ar_count: u16::from_be_bytes(bytes[10..12].try_into().unwrap()),
        };

        let start = std::mem::size_of::<DnsHeader>();
        let mut index = start;
        while index < len && bytes[index] != 0{
            index += 1;
        }

        let name_vec= bytes[start..index+1].to_vec();
        let name = String::from_utf8(name_vec).unwrap();

        index += 1;

        let question = DnsQuestion {
            q_type: u16::from_be_bytes(bytes[index..index+2].try_into().unwrap()),
            q_class: u16::from_be_bytes(bytes[index+2..index+4].try_into().unwrap()),
        };
        index += 4;

        let name_ptr = u16::from_be_bytes(bytes[index..index+2].try_into().unwrap());
        index += 2;

        let answer = DnsQuestion {
            q_type: u16::from_be_bytes(bytes[index..index+2].try_into().unwrap()),
            q_class: u16::from_be_bytes(bytes[index+2..index+4].try_into().unwrap()),
        };
        index += 4;


        let ttl = u32::from_be_bytes(bytes[index..index+4].try_into().unwrap());

        index += 4;

        let addr_len = u16::from_be_bytes(bytes[index..index+2].try_into().unwrap()) as usize;
        index += 2;

        let mut addr = format!("{}.{}.{}.{}", bytes[index], bytes[index+1], bytes[index+2], bytes[index+3]);

        return DnsResponse{
            header: header,
            name: name,
            question: question,
            answer: answer,
            ttl: ttl,
            address: addr
        };
    }
}

pub struct DnsResolver {}

impl DnsResolver {

    pub fn get_host_by_name(host: &str) -> String
    {
        let header_ln2 = Header2{
            qr: false,
            op_code: OpCodeEnum::Query,
            aa: false,
            tc: false,
            rd: false, 
            ra: false,
            r_code:RCodeEnum::NoErr
        };

        let query = DnsQuery {
            header:  DnsHeader::new(
            1337,
            0x0001, 
            1,
            0,
            0,
            0
            ),
            question: DnsQuestion::new(1, 1),
            name: String::from(host)
        };

        let packet = query.as_bytes();

        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        let success = socket.send_to(&packet, "8.8.8.8:53").is_ok();

        let mut buf = [0; 2048];
        let (len, addr) = socket.recv_from(&mut buf).unwrap();
        let response = DnsResponse::from_bytes(&buf, len);

        return response.address;
    }


}