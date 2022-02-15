#[cfg(test)]
mod basic {
    const ARRAY_I32: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
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
	    return binary_search(right_slice, cmp_elem, idx_0+left_slice.len());
	} else {
	    return binary_search(left_slice, cmp_elem, idx_0);
	}
    }


    #[test]
    fn test_linear_search() {
	assert_eq!(linear_search(&ARRAY_STRING, "Rust!" ), 3)
    }

    #[test]
    fn test_binary_search() {
	assert_eq!(binary_search(&ARRAY_I32, 7, 0 as usize), 6)
    }
}
