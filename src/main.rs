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

// Condtionals and early returns
fn collatz_length(mut n:i64) -> u32{
  let mut len = 1;
  while n>1{
    n = if n%2 == 0 {n/2} else {3*n+1};
    len += 1
  }
  len
}

// destructuring tuples and structs and enums 
struct Foo{
  a:i32,
  b:bool,
}
enum FooOrTuple{
  Foo(Foo),
  Tuple(i32,i32),
}
fn tuples(t:FooOrTuple){
  match t{
    FooOrTuple::Foo(f) => {
      println!("Foo: a={}, b={}", f.a, f.b);
    }
    FooOrTuple::Tuple(left,right) => {
      println!("left: {left}, right: {right}");
    }
  }
}

// For loops and arrays and macros  
fn for_loops(){
  // let primes = [2, 3, 5, 7, 11, 13, 17, 19];
  let primes = [2, 3, 5 ];
  
  for prime in primes{
    let mut count = 0;
    for i in 2..prime{
      dbg!(prime,i);
      count += 1;
      dbg!(count);
      assert_ne!(prime % i,0);
    }
  }
}

fn main() {
  // let n = 20;
  // println!("fib({}) = {}", n, fib(n));    
  // println!("factorial({}) = {}", n, factorial(n));
  // println!("collatz_length({}) = {}", n, collatz_length(n));
  // for_loops();
  let foo = Foo {a:12,b:false};
  tuples(FooOrTuple::Tuple(23,25));
  tuples(FooOrTuple::Foo(foo));
}