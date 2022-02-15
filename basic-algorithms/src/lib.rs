#[cfg(test)]
mod basic {
    const ARRAY_I32_SORTED: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    const ARRAY_I32_UNSORTED: [i32; 10] = [3, 10, 1, 5, 9, 4, 7, 8, 6, 2];

    const ARRAY_I32_SORTED2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    const ARRAY_I32_UNSORTED2: [i32; 9] = [3, 1, 5, 9, 4, 7, 8, 6, 2];

    const ARRAY_STRING: [&str; 4] = ["Hello ", "World ", "from", "Rust!"];

    // SEARCHING ALGORITHMS
    // ====================
    // --- linear search O(n)
    // --- binary search O(log2(n))
    fn linear_search<T: Eq>(arr: &[T], cmp_elem: T)
					    -> usize {
	let mut idx: usize = 999;

	for i in 0..arr.len() {
	    if arr[i] == cmp_elem {
		idx = i;
		break;
	    }
	}

	idx
    }
    fn binary_search<T: Ord>(arr: &[T], cmp_elem: T, idx_0: usize)
					     -> usize { // Recursive implementation
	// Split the array using the median element as pivot
	let (left_slice, right_slice) = arr.split_at(arr.len()/2);

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
    // ==================
    // --- bubble sort O(n^2)
    // --- quick sort O(n^2) -> Faster than bubble sort on average
    // --- merge sort O(n*log(n))
    fn bubble_sort<T: Ord>(arr: &mut [T]) {
	let n = arr.len()-1;
	let mut n_swaps: usize;

	loop {
	    n_swaps = 0; // Must become 0 when a new loop cycle begins
	    for i in 0..n {
		if arr[i+1] < arr[i] {
		    arr.swap(i, i+1);
		    n_swaps += 1;
		}
	    }
	    // Exit loop if the array is completely sorted
	    if n_swaps == 0 { break; }
	}
    }
    fn quick_sort<T: Ord>(arr: &mut [T]) { // WORK IN PROGRESS!!!
	let pivot = &arr.len()/2;
	//let pivot_value = arr.get(pivot).unwrap().clone();
	let pivot_value = &arr[pivot];
	let mut n_left: usize = 0;
	let mut n_right: usize = 0;
	for a in arr.as_ref() {
	    if a < pivot_value {
		n_left += 1;
	    }
	    if a > pivot_value {
		n_right += 1;
	    }
	}
    }
    fn merge_sort<T: Ord>(arr: &mut [T]) { // WORK IN PROGRESS!!!
	let mut step: usize;

	for n in 1..arr.len()/2 { // N/2 passes at most
	    step = 2*n; // Slices count twice the step number elements
	    for m in (0..arr.len()).step_by(step) {
		// The last slice can have a lower number of elements
		let limit = std::cmp::min(m+step, arr.len());
		//println!("Working on {:?}", m..limit);
		for p in m..limit {
		    let tmp = &arr[p..limit];
		    println!("Tmp slice: {:?}", p..limit);
		    let mut min_idx = tmp
			.iter()
			.enumerate()
			.min_by(|(_, a), (_, b)| a.cmp(b))
			.map(|(idx, _)| idx)
			.unwrap();
		    // Trnslate from local (slice) to global indexing
		    min_idx += p;
		    println!("Min Idx: {}", min_idx);
		    if p != min_idx {
		    	arr.swap(p, min_idx);
		    }
		}
	    }
	}
    }

    // MISCELLANEA
    fn sieve_of_erathostenes(limit: i32) -> Vec<i32>{
	let mut sieve: Vec<i32> = (2..limit+1).collect();
	let mut pivot: usize = 0; // Index of pivot element
	let mut root: i32; // Value of pivot element
	let mut square: i32; // Squared value of pivot element

	loop {
	    root = sieve[pivot];
	    // NOTE: A better exit condition should be found
	    if root >= sieve[sieve.len()-1] { break; }

	    square = root.pow(2);
	    sieve.retain(|n| n<&square || n%root != 0);
	    pivot += 1;
	}
	sieve
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
	let mut arr = ARRAY_I32_UNSORTED.clone();
	bubble_sort(&mut arr);
	assert_eq!(arr, ARRAY_I32_SORTED);
    }

    //#[test]
    //fn test_quick_sort() {
    //	let mut array = ARRAY_I32_UNSORTED.clone();
    //	quick_sort(&mut array);
    //	assert_eq!(array, ARRAY_I32_SORTED);
    //}

    #[test]
    fn test_merge_sort() {
	let mut arr = ARRAY_I32_UNSORTED.clone();
	merge_sort(&mut arr);
	assert_eq!(arr, ARRAY_I32_SORTED);
    }

    #[test]
    fn test_sieve_of_erathostenes() {
	assert_eq!(sieve_of_erathostenes(42), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41])
    }
}
