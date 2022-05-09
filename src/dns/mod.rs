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

pub struct DnsMessage {

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

        };
    }

    pub fn change_dns_name(host: &str) -> String {
        let mut formatted = String::new();

        let split = host.split(".");
        for s in split {
            formatted = format!("{}{}{}",formatted, s.len(), s);
        }
        return formatted;
    }
}