#[cfg(test)]
mod basic {
    const ARRAY_I32_SORTED: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    const ARRAY_I32_UNSORTED: [i32; 10] = [3, 10, 1, 5, 9, 4, 7, 8, 6, 2];

    const ARRAY_I32_SORTED2: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    const ARRAY_I32_UNSORTED2: [i32; 9] = [3, 1, 5, 9, 4, 7, 8, 6, 2];

    const ARRAY_STRING: [&str; 4] = ["Hello ", "World ", "from", "Rust!"];

    const GRAPH: [[usize; 9]; 9] = [
	[0, 4, 0, 0, 0, 0, 0, 8, 0],
	[4, 0, 8, 0, 0, 0, 0, 11, 0],
	[0, 8, 0, 7, 0, 4, 0, 0, 2],
	[0, 0, 7, 0, 9, 14, 0, 0, 0],
	[0, 0, 0, 9, 0, 10, 0, 0, 0],
	[0, 0, 4, 14, 10, 0, 2, 0, 0],
	[0, 0, 0, 0, 0, 2, 0, 1, 6],
	[8, 11, 0, 0, 0, 0, 1, 0, 7],
	[0, 0, 2, 0, 0, 0, 6, 7, 0]];

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
	if arr.len() > 1 {
	    let length = arr.len();
	    let mut pivot = length/2;
	    let fake = length; // Cannot compare as index in left/right partitions
	    let mut left_test = false;
	    let mut right_test = false;

	    loop {
		println!("pivot: {}", pivot);
		{ // Search for lower elements on the left of pivot and eventually swap
		    let left = &arr[0..pivot];
		    let left_index = left.iter().position(|x| x > &arr[pivot]).unwrap_or(fake);

		    println!("left_index: {}", left_index);

		    if left_index != fake { // Found lower value
			arr.swap(left_index, pivot);
			pivot = left_index;
			println!("pivot becomes: {}", pivot);
		    } else {
			// The left partition is already OK
			left_test = true;
		    }
		}
		{ // Search for higher elements on the right of pivot and eventually swap
		    let right = &arr[pivot+1..length];
		    let right_index = right.iter().position(|x| x < &arr[pivot]).unwrap_or(fake);

		    println!("right_index: {}", right_index+pivot+1);

		    if right_index != fake { // Found higher value
			arr.swap(right_index+pivot+1, pivot); // Pay attention, right partition!
			pivot = right_index+pivot+1; // As above...
			println!("pivot becomes: {}", pivot);
		    } else {
			// The right partition is already OK
			right_test = true;
		    }
		}
		if left_test && right_test { // Apply recursively to partitions
		    quick_sort(&mut arr[0..pivot]);
		    quick_sort(&mut arr[pivot+1..length]);
		    break;
		} else {
		    left_test = false;
		    right_test = false;
		}
	    }
	}
    }
    fn merge_sort<T: Ord>(arr: &mut [T]) {
	let mut step: usize;

	// EX.: [3 10 1 5 9 4 7 8 6 2]
	for n in 1..(arr.len()+1)/2 {
	    // EX.: ....... m=0 .. m=2 . m=4 . m=6 . m=8 .....
	    // .... n=1 => [3 10] [1 5] [9 4] [7 8] [6 2] ....
	    // ............... m=0 ..... m=4 ... m=8 .........
	    // .... n=2 => [3 10 1 5] [4 9 7 8] [6 2] ........
	    // .................... m=0 ....... m=8 ..........
	    // .... n=3 => [1 3 4 5 7 8 9 10 ] [2 6] .........
	    // ...................... m=0 ....................
	    // .... n=4 => [1 2 3 4 5 6 7 8 9 10] ............

	    // This is the length of all slices exect last one
	    step = 2_usize.pow(n as u32);

	    for m in (0..arr.len()).step_by(step) {
		// Remember the last slice can have a lower number of elements
		let limit = std::cmp::min(m+step, arr.len());

		for p in m..limit {
		    // "Extract" the slice to work on
		    let tmp = &arr[p..limit];

		    // Compute index of smaller element in slice
		    let mut min_idx = tmp
			.iter()
			.enumerate()
			.min_by(|(_, a), (_, b)| a.cmp(b))
			.map(|(idx, _)| idx)
			.unwrap();

		    // Translate from local (slice) to global indexing
		    min_idx += p;

		    // Swap elements if first one is not smaller one
		    if p != min_idx {
		    	arr.swap(p, min_idx);
		    }
		}
	    }
	}
    }

    // GRAPHS
    fn dijkstra(arr: &[usize]) {
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

    #[test]
    fn test_quick_sort() {
    	let mut arr = ARRAY_I32_UNSORTED.clone();
    	quick_sort(&mut arr);
    	assert_eq!(arr, ARRAY_I32_SORTED);
    }

    #[test]
    fn test_quick_sort2() {
    	let mut arr = ARRAY_I32_UNSORTED2.clone();
    	quick_sort(&mut arr);
    	assert_eq!(arr, ARRAY_I32_SORTED2);
    }

    #[test]
    fn test_merge_sort() { // Input with even number of element
	let mut arr = ARRAY_I32_UNSORTED.clone();
	merge_sort(&mut arr);
	assert_eq!(arr, ARRAY_I32_SORTED);
    }

    #[test]
    fn test_merge_sort2() { // Input with odd number of element
	let mut arr = ARRAY_I32_UNSORTED2.clone();
	merge_sort(&mut arr);
	assert_eq!(arr, ARRAY_I32_SORTED2);
    }

    #[test]
    fn test_dijkstra() {
    }

    #[test]
    fn test_sieve_of_erathostenes() {
	assert_eq!(sieve_of_erathostenes(42), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41])
    }
}
