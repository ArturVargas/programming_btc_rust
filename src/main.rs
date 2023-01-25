mod field_element {
    pub fn init(num: u32, prime: u32) -> bool {
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        print!("Prime");
        true
    }

    pub fn add(a: u32, b: u32, prime: u32) {
        let first_num = init(a, prime);
        let second_num = init(b, prime);

        println!("First {}", first_num);
        println!("Second {}", second_num);

        let num: u32 = (a + b) % prime;
        print!("Suma {}, Prime {}", num, prime);
    }
}
fn main() {
    println!("Hello, world!");
    field_element::init(10, 11);
    field_element::add(1, 4, 11);
}
