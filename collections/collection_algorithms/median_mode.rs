use std::collections::HashMap;

fn main() {
    let mut v: Vec<u32> = vec![4, 6, 2, 3, 1, 8, 9, 7, 5, 8];

    let v1: Vec<u32> = vec![5, 6, 5, 3, 4, 2, 6, 7, 9, 3, 4, 2, 1, 4, ];

    v.extend(&v1);

    println!("The median and mode for the list are {:?}",  median_mode(&mut v));

}

// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn median_mode(x: &mut [u32]) -> (u32, u32) {
        let mut map = HashMap::new();
        let mut mode = 0;
        let mut freq = 0;
        x.sort_unstable();
        println!("This is the sorted vector {:?}", x);
        let len = x.len();
        let median: u32 = if len % 2 == 0 {
                                let mid1 = x[(len / 2) - 1];
                                let mid2 = x[len / 2];
                                (mid1 + mid2) / 2
                            } else {
                                x[len / 2]
                            };


        for n in x {
            let count = map.entry(*n).or_insert(0);
            *count += 1; 
        }

        for (key, val) in map.iter() {
           println!("key: {}, value: {}", key, val);

           if freq < *val {
            freq = *val;
            mode = *key;
           }
        }

        let result: (u32, u32) = (median, mode);
        result

}