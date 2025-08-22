// DAY2
// Control flow using 'if'

pub fn control_flow() -> () {
    let a = 99;

    if a > 99 {
        println!("big number");
    } else {
        println!("small number ")
    }
}

// Repetition using loops
//'loop' infinite loop
//'while' conditional loop

// the infinite loop

pub fn infinite_loop() {
    let mut a = 0;
    loop {
        if a == 5 {
            break;
        }
        println!("{:?}", a);
        a = a + 1;
    }
}
