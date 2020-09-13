impl Solution {
  pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
    let mut result=vec![1,2,3,4,5,6,7,8,9];
    let mut temp=Vec::<i32>::new();
    
    for i in 1..n {
      for j in result {
        let last=j%10;
        if k==0 {
          temp.push(j*10+last);
        }
        else {
          if last-k>=0 {
            temp.push(j*10+last-k);
          }
          if last+k<10 {
            temp.push(j*10+last+k);
          }
        }
      }
      result=temp.clone();
      temp.clear();
    }
    return result;
  }
}