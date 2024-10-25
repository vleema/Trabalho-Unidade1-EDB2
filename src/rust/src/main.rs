use rand::Rng;
use sort_algorithms::algorithms::*;
use std::{
  fs::{self, File, OpenOptions},
  io::{self, Write},
  time::Instant,
};

const TEN: usize = 10;
const ENTRIES_FILENAME: &str = "out/entries.txt";
const OUTPUT_FILENAME: &str = "out/output.txt";

#[allow(dead_code)]
const ENTRY_SIZE: usize = 1000;

type SortFn<T> = fn(&mut [T]);

fn main() -> io::Result<()> {
  let sort_functions = Vec::from([
    ("ITE BUBBLE SORT", iterative_bubble_sort as SortFn<i32>),
    ("REC BUBBLE SORT", recursive_bubble_sort),
    ("REC QUICK SORT", recursive_quick_sort),
    ("ITE QUICK SORT", iterative_quick_sort),
    ("REC MERGE SORT", recursive_merge_sort),
    ("ITE MERGE SORT", iterative_merge_sort),
  ]);

  fs::create_dir_all("out")?;
  let mut entries_file = File::create(ENTRIES_FILENAME)?;
  let mut output_file = File::create(OUTPUT_FILENAME)?;

  let title = "Performance test for sort algorithms";
  writeln!(output_file, "{}\n{}\n", title, "=".repeat(title.len()))?;
  writeln!(
    output_file,
    "ITE - stands for iterative\nREC - stands for recursive\n"
  )?;

  for n in 1..=4 {
    writeln!(entries_file, "List with {} entries:\n", TEN.pow(n))?;
    run_entry(
      &sort_functions,
      TEN.pow(n),
      &mut entries_file,
      &mut output_file,
    )?;
    writeln!(entries_file)?;
  }

  Ok(())
}

fn run_entry(
  functions: &Vec<(&str, SortFn<i32>)>,
  entry_size: usize,
  entries_file: &mut File,
  output_file: &mut File,
) -> io::Result<()> {
  let gen_start = Instant::now();
  write!(output_file, "Generating list with {} entries", entry_size)?;
  let list = gen_random_list(entry_size);
  writeln!(output_file, " ... ({:?})\n", gen_start.elapsed())?;
  writeln!(entries_file, "{:?}\n", list)?;

  for (func_name, func) in functions {
    writeln!(output_file, "{}:", func_name)?;
    write!(output_file, "  >>> Cloning ... ")?;
    let clone_start = Instant::now();
    let mut list_copy = list.clone();
    writeln!(output_file, "finished in {:?}", clone_start.elapsed())?;

    write!(output_file, "  >>> Running ... ")?;
    let run_start = Instant::now();
    func(&mut list_copy);
    writeln!(output_file, "finished in {:?}\n", run_start.elapsed())?;
  }
  writeln!(output_file, "{}\n", "-".repeat(50))?;

  Ok(())
}

fn gen_random_list(list_size: usize) -> Vec<i32> {
  let mut rng = rand::thread_rng();
  let mut arr = Vec::with_capacity(list_size);
  for _ in 0..list_size {
    arr.push(rng.gen_range(0..100000));
  }
  arr
}
