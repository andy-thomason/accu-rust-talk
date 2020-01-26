
fn main() {
    let fred : Vec<i32> = vec![1, 2, 3, 4];

    println!("fred={:?}", &fred);

    println!("fred.as_slice()={:?}", fred.as_slice());

    let slice = fred.as_slice();

    println!("slice.as_ptr()={:?}", slice.as_ptr());
    println!("slice.len()={:?}", slice.len());
    println!("slice[1]]={:?}", slice[1]);

    let subslice = &slice[1..3];

    println!("subslice={:?}", &subslice);

    for i in subslice.iter() {
        println!("i={}", i);
    }

    let times_two : Vec<_> = subslice.iter().map(|i| i*2).collect();
    println!("times_two={:?}", &times_two);

    let sum : i32 = subslice.iter().sum();
    println!("sum={}", sum);
}
