fn main() {
   let mut x = 1;
   // continue looping until x > 5
   loop { // Similar to while true
      if x > 5 {
         break;
      }
      println!("x = {}", x);
      x += 1;
   }
}
