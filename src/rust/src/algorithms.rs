/// Sorts an array in place using a iterative version of the bubble sort algorithm
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement the `PartialOrd` trait
///
/// # Example
///
/// ```
/// let mut arr = [5, 3, 8, 4, 2];
/// iterative_bubble_sort(&mut arr);
/// assert_eq!(arr, [2, 3, 4, 5, 8]);
/// ```
pub fn iterative_bubble_sort<T: PartialOrd>(arr: &mut [T]) {
  for i in 0..arr.len() {
    for j in 1..arr.len() - i {
      if arr[j - 1] > arr[j] {
        arr.swap(j - 1, j);
      }
    }
  }
}

/// Sorts an array in place using a recursive version of the bubble sort algorithm
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement the `PartialOrd` trait
///
/// # Example
///
/// ```
/// let mut arr = [5, 3, 8, 4, 2];
/// iterative_bubble_sort(&mut arr);
/// assert_eq!(arr, [2, 3, 4, 5, 8]);
/// ```
pub fn recursive_bubble_sort<T: PartialOrd>(arr: &mut [T]) {
  if arr.is_empty() {
    return;
  }
  let last_element_position = arr.len();
  bubble_sort_pass(arr, 1, last_element_position);
  recursive_bubble_sort(&mut arr[..last_element_position - 1]);
}

/// Helper function for the recursive bubble sort algorithm
///
/// This functions moves or "bubbles" the largest element of a slice to the last_element_position
/// in the `arr`.
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement the `PartialOrd` trait
/// * `iterator` - The position of a element in `arr`
/// * `last_element_position` - The position that
///
/// # Example
///
/// ```
/// let mut arr = [6, 9, 1, 2];
/// let n = arr.len();
/// bubble_sort_pass(&mut arr, 1, n);
/// assert_eq!(arr, [6, 1, 2, 9]);
/// ```
fn bubble_sort_pass<T: PartialOrd>(arr: &mut [T], iterator: usize, last_element_position: usize) {
  if iterator >= last_element_position {
    return;
  }
  if arr[iterator - 1] > arr[iterator] {
    arr.swap(iterator - 1, iterator)
  }
  bubble_sort_pass(arr, iterator + 1, last_element_position)
}

/// Wraps the Quick Sort algorithm for sorting an array in place.
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement the `PartialOrd` and `Copy` traits.
///
/// # Example
///
/// ```
/// let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// recursive_quick_sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
/// ```
pub fn recursive_quick_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
  if arr.is_empty() {
    return;
  }
  _recursive_quick_sort(arr, 0, arr.len() - 1);
}

/// Recursively sorts an array in place using the Quick Sort algorithm.
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement the `PartialOrd` and `Copy` traits.
/// * `lower_bound` - The starting index of the slice to be sorted.
/// * `upper_bound` - The ending index of the slice to be sorted.
///
/// This function is called by `quick_sort` and should not be used directly.
///
/// # Example
///
/// ```
/// let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// _recursive_quick_sort(&mut arr, 0, arr.len() - 1);
/// assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
/// ```
fn _recursive_quick_sort<T: PartialOrd + Copy>(
  arr: &mut [T],
  lower_bound: usize,
  upper_bound: usize,
) {
  if lower_bound >= upper_bound {
    return;
  }
  let pivot_index = partition(arr, lower_bound, upper_bound);
  if pivot_index > 0 {
    _recursive_quick_sort(arr, lower_bound, pivot_index - 1);
  }
  _recursive_quick_sort(arr, pivot_index + 1, upper_bound);
}

/// Partitions the array around a pivot element for the Quick Sort algorithm.
///
/// This function selects the last element as the pivot and rearranges the array
/// such that all elements less than the pivot are on the left, and all elements
/// greater than or equal to the pivot are on the right. It then returns the index
/// of the pivot element after partitioning.
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement the `PartialOrd` and `Copy` traits.
/// * `lower_bound` - The starting index of the slice to be partitioned.
/// * `upper_bound` - The ending index of the slice to be partitioned.
///
/// # Returns
///
/// The index of the pivot element after partitioning.
///
/// # Example
///
/// ```
/// let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// let pivot_index = partition(&mut arr, 0, arr.len() - 1);
/// assert_eq!(pivot_index, 4); // Example index, actual value may vary
/// ```
fn partition<T: PartialOrd + Copy>(arr: &mut [T], lower_bound: usize, upper_bound: usize) -> usize {
  let pivot = arr[upper_bound];
  let mut left_item = lower_bound as isize - 1;
  for right_item in lower_bound..upper_bound {
    if arr[right_item] < pivot {
      left_item += 1;
      arr.swap(left_item as usize, right_item);
    }
  }
  let new_pivot = (left_item + 1) as usize;
  arr.swap(new_pivot, upper_bound);
  new_pivot
}

// TODO: Merge sort

#[cfg(test)]
mod tests {
  use super::*;

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

  // #[test]
  // fn test_recursive_merge_sort() {
  //   test_reverse_list(recursive_merge_sort);
  // }
}
