use proconio::input;
use std::fmt;
fn main() {
    input! {
        n: usize,
        b: [i32; 3],
        ls: [[i32; n]; n],
        rs: [[i32; n]; n],
    }
    let mut ans = vec![vec![0; n]; n];
    for (h, col) in ls.iter().enumerate() {
        for (w, &ele) in col.iter().enumerate() {
            ans[h][w] = ele;
        }
    }
    let answer = Answer { ans };
    println!("{}", answer);
}

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
