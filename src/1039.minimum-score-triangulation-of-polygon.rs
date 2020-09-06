use std::cmp;
impl Solution {
  pub fn min_score_triangulation(a: Vec<i32>) -> i32 {
    let n=a.len();
    let mut dp: Vec<Vec<i32>> =vec![vec![-1;n];n];
    Solution::fndp(&a, &mut dp, 0, n-1);
    return dp[0][n-1];
  }

  pub fn fndp(a: &Vec<i32>,dp: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32{
    if dp[i][j]!=-1 {
      return dp[i][j];
    }
    let mut result=std::i32::MAX;
    for k in i+1..j {
      result=cmp::min(result,Solution::fndp(&a, dp, i, k)+a[i]*a[j]*a[k]+Solution::fndp(&a, dp, k, j));
    }
    if result!=std::i32::MAX {
      dp[i][j]=result;
    }
    else {
      dp[i][j]=0;
    }
    return dp[i][j];
  }
}