///

/// Sorts an array in place using a iterative version of the bubble sort algorithm
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement the `PartialOrd` trait
///
/// # Example
///
/// ```rust
/// use sort_algorithms::algorithms::iterative_bubble_sort;
///
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
/// ```rust
/// use sort_algorithms::algorithms::recursive_bubble_sort;
///
/// let mut arr = [5, 3, 8, 4, 2];
/// recursive_bubble_sort(&mut arr);
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
/// ```rust
/// let mut arr = [6, 9, 1, 2];
/// let n = arr.len();
/// bubble_sort_pass(&mut arr, 1, n);
/// assert_eq!(arr, [6, 1, 2, 9]);
/// ```
#[cfg(not(doctest))]
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
/// ```rust
/// use sort_algorithms::algorithms::recursive_quick_sort;
///
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

/// Sorts an array in place using a iterative version of the Quick Sort algorithm
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement the `PartialOrd` and `Copy` traits.
///
/// # Example
///
/// ```rust
/// use sort_algorithms::algorithms::iterative_quick_sort;
///
/// let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// iterative_quick_sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
/// ```
pub fn iterative_quick_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
  if arr.is_empty() {
    return;
  }
  let mut stack = vec![(0, arr.len() - 1)];
  while let Some((low, high)) = stack.pop() {
    if low < high {
      let pivot_index = partition(arr, low, high);
      if pivot_index > 0 {
        stack.push((low, pivot_index - 1)); // Left side
      }
      stack.push((pivot_index + 1, high)); // Right side
    }
  }
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
/// ```rust
/// let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// _recursive_quick_sort(&mut arr, 0, arr.len() - 1);
/// assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
/// ```
#[cfg(not(doctest))]
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
/// ```rust
/// let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// let pivot_index = partition(&mut arr, 0, arr.len() - 1);
/// assert_eq!(pivot_index, 4); // Example index, actual value may vary
/// ```
#[cfg(not(doctest))]
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

/// Sorts an array using a recursive Merge Sort algorithm.
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement `PartialOrd` and `Copy`.
///
/// # Example
///
/// ```rust
/// use sort_algorithms::algorithms::recursive_merge_sort;
///
/// let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// recursive_merge_sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
/// ```
pub fn recursive_merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
  if arr.len() <= 1 {
    return;
  }
  let mid = arr.len() / 2;
  let mut left_arr = arr[..mid].to_vec();
  let mut right_arr = arr[mid..].to_vec();
  recursive_merge_sort(&mut left_arr);
  recursive_merge_sort(&mut right_arr);
  merge(arr, &left_arr, &right_arr);
}

/// Sorts an array using a iterative Merge Sort algorithm
///
/// # Arguments
///
/// * `arr` - A mutable slice of elements that implement `PartialOrd` and `Copy`.
///
/// # Example
///
/// ```rust
/// use sort_algorithms::algorithms::iterative_merge_sort;
///
/// let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
/// iterative_merge_sort(&mut arr);
/// assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
/// ```
pub fn iterative_merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
  if arr.len() <= 1 {
    return;
  }
  let mut temp_arr = arr.to_vec();
  let mut segment_size = 1;
  let arr_len = arr.len();
  while segment_size < arr.len() {
    let mut start = 0;
    while start < arr_len {
      let mid = (start + segment_size).min(arr_len);
      let end = (start + 2 * segment_size).min(arr_len);
      merge(&mut temp_arr[start..end], &arr[start..mid], &arr[mid..end]);
      start += 2 * segment_size;
    }
    arr.copy_from_slice(&temp_arr);
    segment_size *= 2;
  }
}

/// Merges two sorted slices into a single sorted slice.
///
/// # Arguments
///
/// * `arr` - A mutable slice where the merged result will be stored.
/// * `left_arr` - A slice containing the left half of the sorted elements.
/// * `right_arr` - A slice containing the right half of the sorted elements.
///
/// # Example
///
/// ```rust
/// let left_arr = [1, 3, 5];
/// let right_arr = [2, 4, 6];
/// let mut arr = [0; 6];
/// merge(&mut arr, &left_arr, &right_arr);
/// assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
/// ```
#[cfg(not(doctest))]
fn merge<T: PartialOrd + Copy>(arr: &mut [T], left_arr: &[T], right_arr: &[T]) {
  let left_arr_len = left_arr.len();
  let right_arr_len = right_arr.len();
  let (mut i, mut l, mut r) = (0, 0, 0);
  while l < left_arr_len && r < right_arr_len {
    if left_arr[l] < right_arr[r] {
      arr[i] = left_arr[l];
      l += 1;
    } else {
      arr[i] = right_arr[r];
      r += 1;
    }
    i += 1;
  }
  while l < left_arr_len {
    arr[i] = left_arr[l];
    i += 1;
    l += 1;
  }
  while r < right_arr_len {
    arr[i] = right_arr[r];
    i += 1;
    r += 1;
  }
}
