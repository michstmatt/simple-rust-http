use std::str::Split;
pub fn ss_get(iter: &mut Split<&str>) -> String {
    return iter.next().unwrap().into();
}
