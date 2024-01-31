fn main() {
    let thing = 0..100;
    for value in thing {
        print!("{}, ", value);
        if value == 40 {
            panic!("WAHH");
        }
    }
}
