#[cfg(test)]
mod basic {
    const ARRAY_I32_SORTED: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    const ARRAY_I32_UNSORTED: [i32; 10] = [3, 10, 1, 5, 9, 4, 7, 8, 6, 2];
    const ARRAY_STRING: [&str; 4] = ["Hello ", "World ", "from", "Rust!"];

    // SEARCHING ALGORITHMS
    fn linear_search<T:std::cmp::PartialEq>(array: &[T], cmp_elem: T)
					    -> usize {
	let mut idx: usize = 999;

	for i in 0..array.len() {
	    if array[i] == cmp_elem {
		idx = i;
		break;
	    }
	}

	idx
    }
    fn binary_search<T:std::cmp::PartialOrd>(array: &[T], cmp_elem: T, idx_0: usize)
					     -> usize { // Recursive implementation
	// Split the array using the median element as pivot
	let (left_slice, right_slice) = array.split_at(array.len()/2);

	// Index of the pivotal element
	let guess = left_slice.len()-1;

	if left_slice[guess] == cmp_elem {
	    return guess + idx_0;
	} else if left_slice[guess] < cmp_elem {
	    return binary_search(
		right_slice, cmp_elem, idx_0+left_slice.len()
	    );
	} else {
	    return binary_search(
		left_slice, cmp_elem, idx_0
	    );
	}
    }

    // SORTING ALGORITHMS
    fn bubble_sort<T:std::cmp::PartialOrd>(array: &mut [T]) {
	//let mut new_array = array.clone();
	let n = array.len()-1;
	let mut n_swaps: usize;

	loop {
	    n_swaps = 0;
	    for i in 0..n {
		if array[i+1] < array[i] {
		    array.swap(i, i+1);
		    n_swaps += 1;
		}
	    }
	    // Exit loop if the array is completely sorted
	    if n_swaps == 0 { break; }
	}
    }


    #[test]
    fn test_linear_search() {
	assert_eq!(linear_search(&ARRAY_STRING, "Rust!" ), 3)
    }

    #[test]
    fn test_binary_search() {
	assert_eq!(binary_search(&ARRAY_I32_SORTED, 7, 0 as usize), 6)
    }

    #[test]
    fn test_bubble_sort() {
	let mut array = ARRAY_I32_UNSORTED.clone();
	bubble_sort(&mut array);
	assert_eq!(array, ARRAY_I32_SORTED);
    }
}
