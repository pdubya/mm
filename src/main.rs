
use std::collections::HashMap;
use std::thread;

#[derive(Clone, Copy)]
struct Code([u8; 4]);
#[derive(Hash, Clone, Copy)]
struct Response(u8, u8);
const N: usize = usize::pow(6, 4);

fn compare(s: &Code, g: &Code) -> Response  {
    let mut res = [0u8, 0u8];
    let mut accounted = [false; 4];

    for (i, (el_s, el_g)) in s.0.iter().zip(g.0.iter()).enumerate()  {
        if *el_s == *el_g  {
            res[1] += 1;
            accounted[i] = true;
        }
    }

    for (i, el_g) in g.0.iter().enumerate()  {
        if *el_g == s.0[i] { continue }
        for (j, el_s) in s.0.iter().enumerate()  {
            if (!accounted[j]) && (*el_g == *el_s)  {
                res[0] += 1;
                accounted[j] = true;
                break;
            }
        }
    }
    Response(res[0], res[1])
}

struct Decoder  {
    mask: [bool; N],
}

impl Decoder  {
    const fn all_codes() -> [Code; N] {
        let mut a = [Code([0, 0, 0, 0]); N];
        let mut k = 1;
        while k < N  {
            if a[k - 1].0[3] < 5 {
                a[k].0[3] = a[k - 1].0[3] + 1;
                a[k].0[2] = a[k - 1].0[2];
                a[k].0[1] = a[k - 1].0[1];
                a[k].0[0] = a[k - 1].0[0];
            } else if a[k - 1].0[2] < 5 {
                a[k].0[3] = 0;
                a[k].0[2] = a[k - 1].0[2] + 1;
                a[k].0[1] = a[k - 1].0[1];
                a[k].0[0] = a[k - 1].0[0];
            } else if a[k - 1].0[1] < 5 {
                a[k].0[3] = 0;
                a[k].0[2] = 0;
                a[k].0[1] = a[k - 1].0[1] + 1;
                a[k].0[0] = a[k - 1].0[0];
            } else {
                a[k].0[3] = 0;
                a[k].0[2] = 0;
                a[k].0[1] = 0;
                a[k].0[0] = a[k - 1].0[0] + 1;
            }
            k += 1;
        }
        a
    }

    const ALL_CODES: [Code; N] = Decoder::all_codes();
    fn update(self: &mut Self, r: Response, g: Code) -> () {
        for (excluded, c) in self.mask.iter_mut().zip(Decoder::ALL_CODES.iter())  {
            if *excluded { continue }
            let v = compare(&c, &g);
            if v.0 != r.0 || v.1 != r.1  {
                *excluded = true;
            }
        }
    }

    fn next_guess(self: &Self) -> Code {
        let mut min_ent: f64 = (N as f64)*(N as f64);
        let mut res = Code([0, 0, 0, 0]);
        if self.mask.iter().fold(0u32, |v, e| if *e { v } else { v + 1 }) == 1  {
            for (eliminated, c) in self.mask.iter().zip(Decoder::ALL_CODES.iter())  {
                if *eliminated { continue }
                println!(" {}, {}, {}, {}", c.0[0], c.0[1], c.0[2], c.0[3]);
                return *c;
            }
        };

        for (eliminated, c) in self.mask.iter().zip(Decoder::ALL_CODES.iter())  {
            if *eliminated { continue }
            res = *c;
            break;
        }

        for g in Decoder::ALL_CODES.iter()  {
            let mut p = HashMap::new();
            for (eliminated, c) in self.mask.iter().zip(Decoder::ALL_CODES.iter())  {
                if *eliminated { continue }
                let r = compare(c, g);
                let cnt = p.entry((r.0, r.1)).or_insert(0);
                *cnt += 1;
            }
            let next_ent = p.values().map(|v| *v as f64).fold(0f64, |v, n| v + n*n.ln());
            if next_ent < min_ent  {
                println!(" {}, {}, {}, {}: {}, {}", g.0[0], g.0[1], g.0[2], g.0[3], next_ent, p.len());
                res = *g;
                min_ent = next_ent;
            }
        }
        res
    }
}

fn solve(s: &Code) -> Vec<Code>  {
    let mut d = Decoder{mask: [false; N]};
    let mut res = Vec::new();
    loop  {
        let g = d.next_guess();
        let r = compare(s, &g);
        d.update(r, g);
        res.push(g);
        println!("{}/{}", r.0, r.1);
        if r.1 == 4 { break }
    }
    res
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn simple_compare()  {
        let s = Code([0, 0, 0, 0]);
        let g = Code([1, 1, 1, 1]);
        let r = compare(&s, &g);
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 0);
        let s = g;
        let r = compare(&s, &g);
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 4);

    }

    #[test]
    fn onematch_compare()  {
        let s = Code([1, 2, 3, 4]);
        let g = Code([0, 0, 0, 1]);
        let r = compare(&s, &g);
        assert_eq!(r.0, 1);
        assert_eq!(r.1, 0);
        let g = Code([1, 0, 0, 1]);
        let r = compare(&s, &g);
        assert_eq!(r.0, 0);
        assert_eq!(r.1, 1);
    }

    #[test]
    fn mixed_compare()  {
        let s = Code([5, 4, 3, 5]);
        let g = Code([0, 5, 3, 3]);
        let r = compare(&s, &g);
        assert_eq!(r.0, 1);
        assert_eq!(r.1, 1);
    }

    #[test]
    fn first_guess()  {
        let d = Decoder{mask: [false; N]};
        let g = d.next_guess();
        println!(" {}", g.0[0]);
        println!(" {}", g.0[1]);
        println!(" {}", g.0[2]);
        println!(" {}", g.0[3]);
        assert!(g.0[0] != g.0[1]);
        assert!(g.0[0] != g.0[2]);
        assert!(g.0[0] != g.0[3]);
        assert!(g.0[1] != g.0[2]);
        assert!(g.0[1] != g.0[3]);
        assert!(g.0[2] != g.0[3]);
    }

    #[test]
    fn can_solve()  {
        let p = Code([5, 4, 3, 5]);
        let v = solve(&p);
        let s = v.last().expect("no guesses?");
        println!(" {}", s.0[0]);
        println!(" {}", s.0[1]);
        println!(" {}", s.0[2]);
        println!(" {}", s.0[3]);
        assert_eq!(s.0[0], 5);
        assert_eq!(s.0[1], 4);
        assert_eq!(s.0[2], 3);
        assert_eq!(s.0[3], 5);
    }
}


fn main() {
    let mut handles = vec![];
    for i in 0..10  {
        let handle = thread::spawn(move ||  {
            let _v = solve(&Decoder::ALL_CODES[i]);
        });
        handles.push(handle);
    }
    for handle in handles  {
        handle.join().unwrap();
    }
}

