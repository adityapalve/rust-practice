fn fib(n: u32) -> u32{
  if n<=2{
    return 1;
  }else{
    return fib(n-1)+fib(n-2);
  }
}

fn factorial(n: i64) -> i64{
  let mut product = 1;
  for i in 1..=n {
    // product = dbg!(i)*product;
    product *= i
  }
  product
}

fn collatz_length(mut n:i64) -> u32{
  let mut len = 1;
  while n>1{
    n = if n%2 == 0 {n/2} else {3*n+1};
    len += 1
  }
  len
}

fn main() {
  let n = 20;
  // println!("fib({}) = {}", n, fib(n));    
  println!("factorial({}) = {}", n, factorial(n));
  println!("collatz_length({}) = {}", n, collatz_length(n));
}
