fn main() {
    let s1 = solve1(919901);
    for ch in &s1 {
        print!("{}", ch);
    }
    println!("");

    let s2 = solve2(&[9, 1, 9, 9, 0, 1]);
    // let s2 = solve2(&[5, 9, 4, 1, 4]);
    // let s2 = solve2(&[5, 1, 5, 8, 9]);
    println!("2: {}", s2);
}

fn solve1(limit: usize) -> Vec<u8> {
    let mut recip = vec![3u8, 7];
    let mut a = 0;
    let mut b = 1;

    loop {
        let ascore = recip[a] as usize;
        let bscore = recip[b] as usize;
        let sum = ascore + bscore;
        if sum > 9 {
            recip.push((sum / 10) as u8);
        }
        recip.push((sum % 10) as u8);

        a = (a + ascore + 1) % recip.len();
        b = (b + bscore + 1) % recip.len();

        // print it out.
        if false {
            for (i, ch) in recip.iter().cloned().enumerate() {
                if i == a {
                    print!("({})", ch);
                } else if i == b {
                    print!("[{}]", ch);
                } else {
                    print!(" {} ", ch);
                }
            }
            println!("");
        }

        if recip.len() >= limit + 10 {
            break;
        }
    }

    recip[limit .. limit + 10].iter().cloned().collect()
}

fn solve2(pattern: &[u8]) -> usize {
    let mut recip = vec![3u8, 7];
    let mut a = 0;
    let mut b = 1;

    loop {
        let ascore = recip[a] as usize;
        let bscore = recip[b] as usize;
        let sum = ascore + bscore;
        if sum > 9 {
            recip.push((sum / 10) as u8);
            if recip.ends_with(pattern) {
                return recip.len() - pattern.len();
            }
        }
        recip.push((sum % 10) as u8);
        if recip.ends_with(pattern) {
            return recip.len() - pattern.len();
        }

        a = (a + ascore + 1) % recip.len();
        b = (b + bscore + 1) % recip.len();

        // print it out.
        if false {
            for (i, ch) in recip.iter().cloned().enumerate() {
                if i == a {
                    print!("({})", ch);
                } else if i == b {
                    print!("[{}]", ch);
                } else {
                    print!(" {} ", ch);
                }
            }
            println!("");
        }

    }
}
