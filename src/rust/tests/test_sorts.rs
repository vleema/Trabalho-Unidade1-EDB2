use sort_algorithms::algorithms::*;

fn reverse_list_test(func: fn(&mut [i32])) {
  let mut arr = [5, 3, 2, 4, 1];
  func(&mut arr);
  assert_eq!(arr, [1, 2, 3, 4, 5], "reverse_list_test failed");
}

fn duplicates_list_test(func: fn(&mut [i32])) {
  let mut arr = [4, 2, 3, 2, 1, 4];
  func(&mut arr);
  assert_eq!(arr, [1, 2, 2, 3, 4, 4], "duplicates_list_test failed");
}

fn already_sorted_list_test(func: fn(&mut [i32])) {
  let mut arr = [1, 2, 3, 4, 5];
  func(&mut arr);
  assert_eq!(arr, [1, 2, 3, 4, 5], "already_sorted_list_test failed");
}

fn singleton_list_test(func: fn(&mut [i32])) {
  let mut arr = [42];
  func(&mut arr);
  assert_eq!(arr, [42], "singleton_list_test faild");
}

fn empty_list_test(func: fn(&mut [i32])) {
  let mut arr: [i32; 0] = [];
  func(&mut arr);
  assert_eq!(arr, [], "empty_list_test failed");
}

#[test]
fn test_iterative_bubble_sort() {
  reverse_list_test(iterative_bubble_sort);
  duplicates_list_test(iterative_bubble_sort);
  already_sorted_list_test(iterative_bubble_sort);
  singleton_list_test(iterative_bubble_sort);
  empty_list_test(iterative_bubble_sort);
}

#[test]
fn test_recursive_bubble_sort() {
  reverse_list_test(recursive_bubble_sort);
  duplicates_list_test(recursive_bubble_sort);
  already_sorted_list_test(recursive_bubble_sort);
  singleton_list_test(recursive_bubble_sort);
  empty_list_test(recursive_bubble_sort);
}

#[test]
fn test_recursive_quick_sort() {
  reverse_list_test(recursive_quick_sort);
  duplicates_list_test(recursive_quick_sort);
  already_sorted_list_test(recursive_quick_sort);
  singleton_list_test(recursive_quick_sort);
  empty_list_test(recursive_quick_sort);
}

#[test]
fn test_iterative_quick_sort() {
  reverse_list_test(iterative_quick_sort);
  duplicates_list_test(iterative_quick_sort);
  already_sorted_list_test(iterative_quick_sort);
  singleton_list_test(iterative_quick_sort);
  empty_list_test(iterative_quick_sort);
}

#[test]
fn test_recursive_merge_sort() {
  reverse_list_test(recursive_merge_sort);
  duplicates_list_test(recursive_merge_sort);
  already_sorted_list_test(recursive_merge_sort);
  singleton_list_test(recursive_merge_sort);
  empty_list_test(recursive_merge_sort);
}

#[test]
fn test_iterative_merge_sort() {
  reverse_list_test(iterative_merge_sort);
  duplicates_list_test(iterative_merge_sort);
  already_sorted_list_test(iterative_merge_sort);
  singleton_list_test(iterative_merge_sort);
  empty_list_test(iterative_merge_sort);
}
