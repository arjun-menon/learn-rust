fn main() {
    const HELLO: u8 = 5;
    println!("HELLO: {}", HELLO);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    let x1 = 5;
    println!("x1: {}", x1);

    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    let tup1: (i16, f32, u8) = (500, 6.4, 1);
    println!("typ1: {}, {}, {}", tup1.0, tup1.1, tup1.2);

    let arr1: [u8; 5] = [1, 2, 3, 4, 5];
    println!("arr1: {}", arr1[4]);

    let arr2 = ["Nothing"; 3]; // everything initializes to "Nothing", array length: 3
    println!("arr1: {}", arr2[2]);

    another_zinger('a', 1.6, 15);

    let y = {
        let x = 5;
        x + 1
    };
    println!("y: {}", y);

    println!("sooo(): {}", sooo());

    more1();
}

fn sooo() -> i16 {
    let six = 6;
    six + 1
}

fn another_zinger(zinger1: char, zinger2: f32, zinger3: u8) {
    println!("zinger3: {}", zinger3);
}

fn more1() {
    let number = 5;
    let some = if true { 5 } else { 6 };

    if number < some {
        println!("number < some");
    } else if number == some {
        println!("number == some");
    } else {
        println!("number > some");
    }

    print!("i: ");
    let mut i: u8 = 5;
    let ret = 'counting_down: loop {
        if i == 1 {
            loop {
                break 'counting_down "yes"; // the loop expression evaluates to this value.
                                            // break 'label EXPR; <-- EXPR becomes the loop's value
                                            // see: https://github.com/rust-lang/rfcs/blob/master/text/1624-loop-break-value.md#detailed-design
            }
        }
        print!("{} ", i);
        i -= 1;
    };
    println!("{} {}", i, ret);

    let mut i = 3;
    while i != 0 {
        i -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    print!("a: "); // round 1
    while index < a.len() {
        print!("{} ", a[index]);
        index += 1;
    }
    println!();

    print!("a: "); // round 2
    for element in a {
        print!("{} ", element);
    }
    println!();

    print!("1 to 7 in reverse: "); // round 3
    for number in (1..(7 + 1)).rev() {
        print!("{} ", number);
    }
    println!();

    more2();
}

fn more2() {
    //
}
