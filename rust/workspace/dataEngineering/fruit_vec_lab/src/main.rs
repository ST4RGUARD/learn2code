fn main() {
  let mut fruit_salad = vec!["apple", "banana", "cherry"];
  fruit_salad.push("pear"); 
  fruit_salad.pop();

  for i in fruit_salad {
        println!("i'm a {i}")
    }
}
