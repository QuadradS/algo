use dotenv::dotenv;
use std::env;

#[test]
fn test_bynary_search() {

    dotenv().ok();

    let target : i32 = std::env::var("BINARY_SEARCH_TARGET")
        .expect("BINARY_SEARCH_TARGET not set")
        .parse()
        .expect("Failed to parse BINARY_SEARCH_TARGET as i32");

    let lim: i32 = std::env::var("BINARY_SEARCH_LIM")
        .expect("BINARY_SEARCH_LIM not set")
        .parse()
        .expect("Failed to parse BINARY_SEARCH_LIM as i32");

    println!("------------------------------");
    println!("Search {}", target);
    println!("Range between 0 and {}", lim);
    println!("------------------------------");



    let arr: Vec<i32> = (0..lim).collect::<Vec<i32>>();

    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut mid;
    let mut count = 0;

    if target > (arr.len() - 1) as i32 {
        panic!("target not in range aff arr")
    }

    loop {
        mid = (high + low) / 2;
       
        println!("STEP: {}", count);
        println!("POSITION {}", mid);
        println!("------------------------------");

        count += 1;

        

        if arr[mid] == target {
            break;
        }

        if arr[mid] > target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    println!("RESULTS:");
    println!("");
    println!("ARRAY LENGTH: {}", arr.len());
    println!("TOTAL STEPS COUNTS: {}", count);
    println!("INDEX: {}", mid);
    println!("VALUE: {}", arr[mid]);
    println!("------------------------------");
}
