mod field_element {
    pub fn init(num: u32, prime: u32) -> bool {
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        true
    }
}
