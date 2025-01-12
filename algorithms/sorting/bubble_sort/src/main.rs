use std::i64;

fn bubble_sort(mut arr: Vec<i64>) -> Vec<i64> {
    let l = arr.len();
    println!("length of array: is {length}", length = l);

    print!("Array Values: ");
    for i in 0..arr.len() {
        print!("{}, ", arr[i]);
    }

    for j in 0..arr.len() - 1 {
        let mut swapped = false;
        for i in 0..arr.len() - j - 1 {
            if arr[i] > arr[i + 1] {
                let tmp = arr[i];
                arr[i] = arr[i + 1];
                arr[i + 1] = tmp;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }

    println!("\nBubble Sorted array: {:?}", arr);
    return arr
}

fn main() {
    println!("Starting the program!!!");
    let ip_1: Vec<i64> = vec![23, 1, 12, 10, 7, 0, 123];
    let ip_1 = bubble_sort(ip_1);
    println!("ip_1 {:?}: ", ip_1);
}
