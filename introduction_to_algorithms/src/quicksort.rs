fn quicksort<const LEN: usize>(arr: &mut [usize; LEN], p: usize, r: usize) -> [usize; LEN] {
    if(arr.len() == 0) {
        return arr;
    }

    if(r > arr.len()) {
        r = arr.len();
    }

    if(p <= 0) {
        p = 1;
    }

    if(p >= r){
        return arr;
    }

    let q : usize = partition(arr, p, r);
    quicksort(arr, p, q - 1);
    quicksort(arr, q + 1, r);

    return arr;
}

fn partition(arr: &mut [usize], p: usize, r: usize) -> usize {
    let x : usize = arr[r];
    let mut i : usize = p - 1;
    let mut swap : usize;

    for j in p..r {
        if(arr[j] <= x) {
            i = i + 1;
            swap = arr[i];
            arr[i] = arr[j];
            arr[j] = swap;
        }
    }
    swap = arr[i];
    arr[i] = arr[r];
    arr[r] = swap;
    return i + 1; 
}