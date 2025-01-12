
fn selection_sort(mut arr: Vec<i64>) -> Vec<i64> {
    let l = arr.len();
    println!("length of array is {l}");
    for i in 0..arr.len() - 1 {

        let mut min_i = i;
        for j in i+1..arr.len() {
            if arr[j] < arr[min_i] {
                min_i = j;
            }
        }
        let tmp = arr[min_i];
        arr[min_i] = arr[i];
        arr[i] = tmp;
    }
    return arr;
    
}

fn main() {
    let arr = vec![64, 25, 12, 22, 11];
    let op = selection_sort(arr);
    println!("Output array after selection Sort: {:?}", op)
    
}
