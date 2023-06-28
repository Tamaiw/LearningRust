mod quicksort;

fn main() {
    let arr: [u16; 7] = [0,1,6,3,4,45,5];
    println!("Array {:?}", arr);
    let arr2: [usize] = quicksort::quicksort(arr, 1, arr.len());
    println!("Array {:?}", arr);
}

