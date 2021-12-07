struct Perm {
    res: Vec<Vec<u8>>,
    n: i32,
}

fn fact(n: usize) -> usize {
    let mut res = 1;
    for i in 2..n+1 {
        res *= i;
    }
    res
}

impl Perm {
    fn new(n: i32) -> Perm {

        Perm {
            n,
            res: Vec::with_capacity(fact(n as usize))
        }
    }

    fn rec(&mut self, done: i32, s: Vec<u8>) {
        if done == (1<<self.n)-1 {
            self.res.push(s);
            return;
        }

        for i in 0..self.n {
            if 1<<i&done > 0 {
                continue;
            }

            let c = b'A' + i as u8;
            let mut s2 = Vec::with_capacity(s.len()+1);
            s2.extend_from_slice(&s);
            s2.push(c);
            self.rec(done|1<<i, s2);
        }
    }
}

fn main() {
    let mut perm = Perm::new(10);

    perm.rec(0, Vec::new());
    println!("Size: {}", perm.res.len());
    for s in perm.res.iter().take(5) {
        println!("{:?}", s);
    }
}
