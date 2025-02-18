pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.max {
            return None;
        }

        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

pub fn loop_iterator() {
    let mut stepper1 = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };
    loop {
        match stepper1.next() {
            Some(val) => println!("Loop val: {val}"),
            None => break,
        }
    }

    let mut n = 0;
    while n < 10 {
        n += 1;
        println!("While loop n: {}", n);
    }

    for i in 0..10 {
        println!("For loop i: {}", i);
    }
}
