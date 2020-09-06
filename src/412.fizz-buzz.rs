impl Solution {
  pub fn fizz_buzz(n: i32) -> Vec<String> {
      let mut ans: Vec<String>=Vec::new();
      for i in 1..=n {
          if i%15==0 {
              ans.push(String::from("FizzBuzz"));
          } else if i%3==0 {
              ans.push(String::from("Fizz"));
          } else if i%5==0 {
              ans.push(String::from("Buzz"));
          } else {
              ans.push(i.to_string());
          }
      }
      return ans;
  }
}