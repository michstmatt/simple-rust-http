use std::str::Split;
pub fn ss_get(iter: &mut Split<&str>) -> String {
    let s = iter.next();
    return if s.is_some() {
        s.unwrap().into()
    }
    else{
        String::new()
    };
}
