use std::cmp;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut mm=m;
        let mut nn=n;
        for i in ops.iter(){
            mm=cmp::min(mm,i[0]);
            nn=cmp::min(nn,i[1]);
        }
        let ans=mm*nn;
        return ans;
    }
}