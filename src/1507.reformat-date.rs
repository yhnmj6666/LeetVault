impl Solution {
  pub fn reformat_date(date: String) -> String {
    let trims: &[_]=&['s','t','n','d','r','h'];
    let components: Vec<&str>=date.split(' ').collect();
    let day: i32=components[0].trim_end_matches(trims).parse().unwrap();
    let month: i32=match components[1] {
      "Jan"=>1,
      "Feb"=>2,
      "Mar"=>3,
      "Apr"=>4,
      "May"=>5,
      "Jun"=>6,
      "Jul"=>7,
      "Aug"=>8,
      "Sep"=>9,
      "Oct"=>10,
      "Nov"=>11,
      "Dec"=>12,
      _ => 0,
    };
    let year=components[2];
    return format!("{}-{:02}-{:02}",year,month,day);
  }
}