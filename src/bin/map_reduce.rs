// Testcase: map-reduce

use std::thread;

fn main() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    let mut children = vec![];

    #[allow(unused_doc_comments)]
    /**
     * "Map" phase
     */
    let chunked_data = data.split_whitespace();

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        children.push(thread::spawn(move || {
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segent{}, result={}", i, result);
            result
        }));
    }

    #[allow(unused_doc_comments)]
    /**
     * Reduce phase
     */
    let mut intermediate_sums = vec![];
    for child in children {
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    // let final_result = intermediate_sums.iter().sum::<u32>();

    // ToDo : try without turbofish
    let final_result: u32 = intermediate_sums.iter().sum();
    println!("Final sum result: {}", final_result);
}
