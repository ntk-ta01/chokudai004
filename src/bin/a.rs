use proconio::input;
use rand::Rng;
use std::cmp;
use std::fmt;

const TIMELIMIT: f64 = 2.955;
fn main() {
    // let time = Timer::new();
    input! {
        n: usize,
        b: [i32; 3],
        ls: [[i32; n]; n],
        rs: [[i32; n]; n],
    }
    let input = Input { n, b, ls, rs };
    let ans = vec![vec![0; input.n]; input.n];
    // for (h, col) in input.ls.iter().enumerate() {
    //     for (w, &ele) in col.iter().enumerate() {
    //         ans[h][w] = ele;
    //     }
    // }
    let mut answer = Answer { ans };
    let score = beam_search(&mut answer, &input);
    // let score = simulated_annealing(&mut answer, &input, time);
    println!("{}", answer);
    eprintln!("{}", score);
    // eprintln!("{}", time.get_time());
}

#[allow(dead_code)]
fn beam_search(ans: &mut Answer, input: &Input) -> i32 {
    let mut states = vec![State {
        state: (ans.compute_score(&input.b), ans.clone()),
    }];
    let beam_width = 120;
    for h in 0..input.n {
        for w in 0..input.n {
            let mut new_states = vec![];
            while !states.is_empty() {
                let state = states.pop().unwrap();
                let mut score = state.state.0;
                let mut ans = state.state.1;
                for num in input.ls[h][w]..input.rs[h][w] + 1 {
                    score = ans.adjust_score(score, num, h, w, &input.b);
                    new_states.push(State {
                        state: (score, ans.clone()),
                    });
                }
            }
            states = new_states;
            if beam_width < states.len() {
                states.sort_by(|a, b| a.cmp(b).reverse());
                states = states[..beam_width].to_vec();
            }
        }
    }
    *ans = states[0].state.1.clone();
    states[0].state.0
}

#[allow(dead_code)]
fn simulated_annealing(ans: &mut Answer, input: &Input, time: Timer) -> i32 {
    let mut rng = rand_pcg::Pcg64Mcg::new(854091);

    const STARTTEMP: f64 = 5e4;
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
        let new_score = ans.adjust_score(score, change_num, h, w, &input.b);
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

#[derive(Clone, Eq, PartialEq)]
struct Answer {
    ans: Vec<Vec<i32>>,
}

impl Answer {
    /// O(n^3)で計算したスコアを返す
    fn compute_score(&self, b: &Vec<i32>) -> i32 {
        let n = self.ans.len();
        let mut score = 0;
        for h in 0..n {
            for w in 0..n {
                let mut now = 0;
                for i in w..n {
                    if self.ans[h][w] == 0 {
                        continue;
                    }
                    now += self.ans[h][i];
                    if now == b[0] {
                        score += b[0];
                    }
                    if now == b[1] {
                        score += b[1];
                    }
                    if now == b[2] {
                        score += b[2];
                    }
                    if now > b[2] {
                        break;
                    }
                }
            }
        }
        for h in 0..n {
            for w in 0..n {
                let mut now = 0;
                for i in h..n {
                    if self.ans[h][w] == 0 {
                        continue;
                    }
                    now += self.ans[i][w];
                    if now == b[0] {
                        score += b[0];
                    }
                    if now == b[1] {
                        score += b[1];
                    }
                    if now == b[2] {
                        score += b[2];
                    }
                    if now > b[2] {
                        break;
                    }
                }
            }
        }
        score
    }

    fn adjust_score(&mut self, score: i32, chnum: i32, ch: usize, cw: usize, b: &Vec<i32>) -> i32 {
        let n = self.ans.len();
        let mut loss_score = 0;
        for w in 0..n {
            let mut now = 0;
            for i in w..n {
                if self.ans[ch][i] == 0 {
                    continue;
                }
                now += self.ans[ch][i];
                if now == b[0] {
                    loss_score += b[0];
                }
                if now == b[1] {
                    loss_score += b[1];
                }
                if now == b[2] {
                    loss_score += b[2];
                }
                if now > b[2] {
                    break;
                }
            }
        }

        for h in 0..n {
            let mut now = 0;
            for i in h..n {
                if self.ans[i][cw] == 0 {
                    continue;
                }
                now += self.ans[i][cw];
                if now == b[0] {
                    loss_score += b[0];
                }
                if now == b[1] {
                    loss_score += b[1];
                }
                if now == b[2] {
                    loss_score += b[2];
                }
                if now > b[2] {
                    break;
                }
            }
        }

        self.ans[ch][cw] = chnum;

        let mut get_score = 0;
        for w in 0..n {
            let mut now = 0;
            for i in w..n {
                if self.ans[ch][i] == 0 {
                    continue;
                }
                now += self.ans[ch][i];
                if now == b[0] {
                    get_score += b[0];
                }
                if now == b[1] {
                    get_score += b[1];
                }
                if now == b[2] {
                    get_score += b[2];
                }
                if now > b[2] {
                    break;
                }
            }
        }

        for h in 0..n {
            let mut now = 0;
            for i in h..n {
                if self.ans[i][cw] == 0 {
                    continue;
                }
                now += self.ans[i][cw];
                if now == b[0] {
                    get_score += b[0];
                }
                if now == b[1] {
                    get_score += b[1];
                }
                if now == b[2] {
                    get_score += b[2];
                }
                if now > b[2] {
                    break;
                }
            }
        }

        score - loss_score + get_score
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
#[derive(Clone, Eq)]
struct State {
    state: (i32, Answer),
}

impl cmp::Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.state.0.cmp(&other.state.0)
    }
}

impl cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.state.0 == other.state.0
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
    #[allow(dead_code)]
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
