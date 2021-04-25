use proconio::input;
use rand::Rng;
use std::fmt;

const TIMELIMIT: f64 = 2.955;
fn main() {
    let time = Timer::new();
    input! {
        n: usize,
        b: [i32; 3],
        ls: [[i32; n]; n],
        rs: [[i32; n]; n],
    }
    let input = Input { n, b, ls, rs };
    let mut ans = vec![vec![0; input.n]; input.n];
    for (h, col) in input.ls.iter().enumerate() {
        for (w, &ele) in col.iter().enumerate() {
            ans[h][w] = ele;
        }
    }
    let mut answer = Answer { ans };
    let score = simulated_annealing(&mut answer, &input, time);
    println!("{}", answer);
    eprintln!("{}", score);
}

fn simulated_annealing(ans: &mut Answer, input: &Input, time: Timer) -> i32 {
    let mut rng = rand_pcg::Pcg64Mcg::new(854091);

    const STARTTEMP: f64 = 2e4;
    const ENDTEMP: f64 = 0.1;

    let mut temp = STARTTEMP;
    let mut prob: f64;

    let mut score = ans.compute_score(&input.b);

    let mut best_score = score;
    let mut best_answer = ans.clone();

    let mut loop_count = 0;

    loop {
        loop_count += 1;
        if loop_count >= 100 {
            loop_count = 0;
            let passed = time.get_time() / TIMELIMIT;
            if passed >= 1.0 {
                // println!("{} {}", temp, score);
                break;
            }
            temp = STARTTEMP.powf(1.0 - passed) * ENDTEMP.powf(passed);
        }

        // ランダムに1マス選んで
        // ランダムに数を変更
        let h = rng.gen_range(0, input.n);
        let w = rng.gen_range(0, input.n);
        let change_num = rng.gen_range(input.ls[h][w], input.rs[h][w] + 1);
        let before_num = ans.ans[h][w];
        ans.ans[h][w] = change_num;
        let new_score = ans.compute_score(&input.b);
        prob = f64::exp((new_score - score) as f64 / temp);

        if score <= new_score || (new_score > 0 && rng.gen_bool(prob)) {
            score = new_score;
        } else {
            ans.ans[h][w] = before_num;
        }

        if best_score < score {
            best_score = score;
            best_answer = ans.clone();
        }
    }
    *ans = best_answer;
    best_score
}

struct Input {
    n: usize,
    b: Vec<i32>,
    ls: Vec<Vec<i32>>,
    rs: Vec<Vec<i32>>,
}

#[derive(Clone)]
struct Answer {
    ans: Vec<Vec<i32>>,
}

impl Answer {
    /// O(n^3)で計算したスコアを返す
    fn compute_score(&self, b: &Vec<i32>) -> i32 {
        let n = self.ans.len();
        let mut a = vec![0; 3];
        for h in 0..n {
            let mut cul = vec![0; n + 1]; // 累積和配列
            for i in 0..n {
                cul[i + 1] = self.ans[h][i] + cul[i];
            }
            for wl in 0..n {
                for wr in wl + 1..n + 1 {
                    let num_sum = cul[wr] - cul[wl];
                    for (idx, &like_num) in b.iter().enumerate() {
                        if like_num == num_sum {
                            a[idx] += 1;
                        }
                    }
                }
            }
        }
        for w in 0..n {
            let mut cul = vec![0; n + 1];
            for i in 0..n {
                cul[i + 1] = self.ans[i][w] + cul[i];
            }
            for hl in 0..n {
                for hr in hl + 1..n + 1 {
                    let num_sum = cul[hr] - cul[hl];
                    for (idx, &like_num) in b.iter().enumerate() {
                        if like_num == num_sum {
                            a[idx] += 1;
                        }
                    }
                }
            }
        }
        a.iter().zip(b.iter()).map(|(a, b)| a * b).sum()
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::from("");
        for (h, col) in self.ans.iter().enumerate() {
            for (w, &ele) in col.iter().enumerate() {
                ret.push_str(&ele.to_string());
                if w != col.len() - 1 {
                    ret.push(' ');
                }
            }
            if h != self.ans.len() - 1 {
                ret.push('\n');
            }
        }
        write!(f, "{}", ret)
    }
}

pub fn get_time() -> f64 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9
}

struct Timer {
    start_time: f64,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            start_time: get_time(),
        }
    }

    fn get_time(&self) -> f64 {
        get_time() - self.start_time
    }
}

#[cfg(test)]
mod tests {
    use super::Answer;
    #[test]
    fn computing_score() {
        let answer = Answer {
            ans: vec![vec![5, 5, 5], vec![5, 0, 0], vec![5, 0, 0]],
        };
        assert_eq!(answer.compute_score(&vec![10, 20, 30]), 40)
    }
}
