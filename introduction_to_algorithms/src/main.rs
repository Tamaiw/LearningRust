fn main() {
    let arr = [1,4,3,4,10,0,7,5,8,4,4234,345];
    println!("Unsorted array: {:?}", arr);
    println!("Sorted array: {:?}", quicksort(arr, 0, (arr.len()-1) as u32));
}

fn quicksort<const LEN: usize>(arr: [i32; LEN], lo: u32, hi: u32) ->[i32; LEN] {
    println!("lo is: {:?} + hi is: {:?}", lo, hi);
    if lo >= hi{
        return arr;
    }
    let (p, array2) = partition(arr, lo, hi);    
    let array3 = quicksort(array2, lo, p as u32);
    let result = quicksort(array3, (p+1) as u32, hi);

    
    return result;
}

fn partition<const LEN: usize>(mut arr: [i32; LEN], lo: u32, hi: u32) -> (usize, [i32; LEN]) {
    let middle_of_array = ((hi - lo)/2 + lo) as usize;
    let pivot = arr[middle_of_array];

    let mut left_index = lo as usize;
    let mut right_index = hi as usize ;

    loop {
        while arr[left_index] < pivot {
            left_index = left_index + 1;
        }
        while arr[right_index] > pivot {
            right_index = right_index - 1;
        }
        if (left_index >= right_index) || (arr[left_index] == pivot && arr[right_index] == pivot) {
            return (right_index, arr);
        }
        arr.swap(left_index, right_index);
    }
}
