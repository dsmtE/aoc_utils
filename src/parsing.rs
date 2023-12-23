pub fn to_decimal(c: char) -> Option<u32> {
    let as_number = (c as u32).wrapping_sub('0' as u32);
    if as_number < 10 { Some(as_number) } else { None }
}