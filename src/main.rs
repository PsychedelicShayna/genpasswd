pub struct MT19937 {
    l: u128,
    u: u128,
    n: usize,
    a: u128,
    r: u128,
    f: u128,
    b: u128,
    m: usize,
    d: u128,
    w: u128,
    c: u128,
    s: u128,
    t: u128,
    x: Vec<u128>,
    cnt: usize,
}

impl MT19937 {
    pub fn new(seed: u128) -> Self {
        let mut mt = MT19937 {
            w: 32,
            n: 624,
            f: 1812433253,
            m: 397,
            r: 31,
            a: 0x9908B0DF,
            d: 0xFFFFFFFF,
            b: 0x9D2C5680,
            c: 0xEFC60000,
            u: 11,
            s: 7,
            t: 15,
            l: 18,
            x: vec![0; 624],
            cnt: 0,
        };

        mt.initialize(seed);
        mt
    }

    fn initialize(&mut self, seed: u128) {
        self.x[0] = seed;

        let max_value = u128::MAX;
        
        for i in 1..self.n {
            self.x[i] = (self
                .f
                .wrapping_mul(self.x[i - 1] ^ (self.x[i - 1] >> (self.w - 2)))
                .wrapping_add(i as u128))
                & max_value;
        }

        self.twist();
    }

    fn twist(&mut self) {
        let lower_mask = if self.r < 32 {
            (1u128 << self.r) - 1
        } else {
            u128::MAX
        };

        let upper_mask = !lower_mask & u128::MAX;

        for i in 0..self.n {
            let tmp = (self.x[i] & upper_mask) + (self.x[(i + 1) % self.n] & lower_mask);
            let mut tmp_a = tmp >> 1;

            if tmp % 2 != 0 {
                tmp_a ^= self.a;
            }

            self.x[i] = self.x[(i + self.m) % self.n] ^ tmp_a;
        }
        self.cnt = 0;
    }

    pub fn temper(&mut self) -> u128 {
        if self.cnt == self.n {
            self.twist();
        }

        let mut y = self.x[self.cnt];

        y ^= (y >> self.u) & self.d;
        y ^= (y << self.s) & self.b;
        y ^= (y << self.t) & self.c;
        y ^= y >> self.l;

        self.cnt += 1;

        y & u128::MAX
    }

    pub fn coinflip(&mut self) -> bool {
        self.temper() % 2 == 0
    }
}

const CHARSET_UPPER: usize = 0x01;
const CHARSET_LOWER: usize = 0x02;
const CHARSET_NUMERIC: usize = 0x04;
const CHARSET_SPECIAL: usize = 0x08;

fn passgen(charsets_enabled: usize, length: usize) -> String {
    let mut rng = MT19937::new(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
    );

    for _ in 0..26 {
        rng.temper();
    }

    let mut charsets: Vec<String> = vec![];

    if charsets_enabled & CHARSET_UPPER != 0 {
        charsets.push("ABCDEFGHIJKLMNOPQRSTUVWXYZ".into());
    }

    if charsets_enabled & CHARSET_LOWER != 0 {
        charsets.push("abcdefghijklmnopqrstuvwxyz".into());
    }

    if charsets_enabled & CHARSET_NUMERIC != 0 {
        charsets.push("1234567890".into());
    }

    if charsets_enabled & CHARSET_SPECIAL != 0 {
        charsets.push("`~!@#$%^&*()_+-=[]{}\\;'\"".into());
    }

    let mut randstr = String::new();

    for _ in 0..length {
        let charset = &charsets[rng.temper() as usize % charsets.len()];
        let index = rng.temper() as usize % charset.len().saturating_sub(1);
        randstr.push(charset.chars().nth(index).unwrap());
    }

    randstr
}

fn main() {
    let mut charsets = 0;
    let mut length = 32;

    for arg in std::env::args().skip(1) {
        if arg == "--help" || arg == "-h" {
            println!("Usage: passgen [charclasses|length]
Where charclasses can be:
    u = uppercase
    l = lowercase
    a = alphabetical (both)
    n | d = numerical or digits
    s | x = special or extra

Example:
    passgen 10 ul         10 Upper and lowercase.
    passgen 10 ns         10 Numerical and special characters.

Order is irrelevant:
    passgen ld 10         10 Lowercase and digits.

Default (no args):
    passgen 32 nax        32 Alphabetic numeric and special (default)");


            return;
        } else if arg.chars().all(|c| c.is_ascii_digit()) {
            length = arg.parse::<usize>().unwrap();
            continue;
        } else {
            for c in arg.chars() {
                let cset = match c {
                    'u' => CHARSET_UPPER,
                    'l' => CHARSET_LOWER,
                    'a' => CHARSET_UPPER | CHARSET_LOWER,
                    'n' | 'd' => CHARSET_NUMERIC,
                    's' | 'x' => CHARSET_SPECIAL,
                    _ => 0,
                };

                charsets |= cset;

            }
        }
    }


    if charsets == 0 {
        charsets = CHARSET_UPPER | CHARSET_LOWER | CHARSET_NUMERIC | CHARSET_SPECIAL;
    }

    let pass = passgen(charsets, length);

    println!("{}", &pass);
}
