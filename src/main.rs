// use std::process::Command;

fn main() {
  // let mut cmd = Command::new("ruby");

  // cmd.arg("ruby.rb");

  // match cmd.output() {
  //   Ok(o) => unsafe {
  //     println!("Output: {}", String::from_utf8_unchecked(o.stdout));
  //   },
  //   Err(e) => {
  //     println!("There was an error {}", e);
  //   }
  // }

  println!("Hello, world!");

  let test = 90;
  assert_eq!(test, 90);
  println!("{}", test);

  for i in 0..599999999 {
    if i % 2 == 0 {
      println!("even {}", i);
    } else {
      println!("odd {}", i);
    }
  }
}
