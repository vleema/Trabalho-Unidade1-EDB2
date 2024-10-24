use rand::Rng;
use sort_algorithms::algorithms::*;
use std::{io::Write, time::Instant};

const TEN: usize = 10;

#[allow(dead_code)]
const ENTRY_SIZE: usize = 1000;

type SortFn<T> = fn(&mut [T]);

fn main() {
  let sort_functions = Vec::from([
    ("ITE BUBBLE SORT", iterative_bubble_sort as SortFn<i32>),
    ("REC BUBBLE SORT", recursive_bubble_sort),
    ("REC QUICK SORT", recursive_quick_sort),
    ("ITE QUICK SORT", iterative_quick_sort),
    ("REC MERGE SORT", recursive_merge_sort),
  ]);

  let title = "Performance test for sort algorthms";
  println!("{}\n{}\n", title, "=".repeat(title.len()));

  for n in 1..=4 {
    run_entry(&sort_functions, TEN.pow(n));
  }
  // run_entry(&sort_functions, ENTRY_SIZE);
  //
  println!("ITE - stands for iterative\nREC - stands for recursive");
}

fn run_entry(functions: &Vec<(&str, SortFn<i32>)>, entry_size: usize) {
  let gen_start = Instant::now();
  print!("Generating list with {} entries", entry_size);
  let list = gen_random_list(entry_size);
  println!(" ... ({:?})\n", gen_start.elapsed());

  for (func_name, func) in functions {
    println!("{}:", func_name);
    print!("  >>> Cloning ... ");
    std::io::stdout().flush().unwrap();
    let clone_start = Instant::now();
    let mut list_copy = list.clone();
    println!("finished in {:?}", clone_start.elapsed());

    print!("  >>> Running ... ");
    std::io::stdout().flush().unwrap();
    let run_start = Instant::now();
    func(&mut list_copy);
    println!("finished in {:?}\n", run_start.elapsed())
  }
  println!("{}\n", "-".repeat(50));
}

fn gen_random_list(size: usize) -> Vec<i32> {
  let mut rng = rand::thread_rng();
  let mut arr = Vec::with_capacity(size);
  for _ in 0..size {
    arr.push(rng.gen::<i32>());
  }
  arr
}
