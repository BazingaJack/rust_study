pub fn bubble_sort(arr: &mut [i32]) -> (&mut[i32]){
    let mut swapped = true;
    let mut len = arr.len();
    for i in 0..len{
        swapped = false;
        for j in 0..len-i-1{
            if arr[j] > arr[j+1]{
                arr.swap(j, j+1);
                swapped = true; 
            }
        }
        if(swapped == false){
            break;
        }
    }
    return arr;
}
fn main() {
    let mut v = vec![1,8,6,3,5,4,7,9,2];
    println!("Before Sort:{:?}",v);
    bubble_sort(&mut v);
    println!("After Sort:{:?}",v);
}