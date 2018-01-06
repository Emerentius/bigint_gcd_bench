extern crate num_bigint;
extern crate mod_num_bigint;
extern crate num;
extern crate time;
extern crate rand;

use rand::Rng;
use num::integer::Integer;

fn main() {
    let mut rng = rand::StdRng::new().unwrap();

    println!("2^32n bits\teuclidean gcd\tbinary gcd\tspeedup");
    for n in 1..10+1 {
        let mut nums = vec![];
        let mut nums2 = vec![];
        for _ in 0..300 {
            let bits = rng.gen_iter()
                .take(n)
                .collect::<Vec<_>>();
            nums.push(num_bigint::BigUint::new(bits.clone()));
            nums2.push(mod_num_bigint::BigUint::new(bits));
        }

        let mut gcds = vec![];
        let duration;
        {
            let start = time::precise_time_s();
            for (i, num1) in nums.iter().enumerate() {
                for num2 in nums[i+1..].iter() {
                    gcds.push(num1.gcd(&num2));
                }
            }
            let end = time::precise_time_s();
            duration = end - start;
        }

        let mut gcds2 = vec![];
        let duration2;
        {
            let start = time::precise_time_s();
            for (i, num1) in nums2.iter().enumerate() {
                for num2 in nums2[i+1..].iter() {
                    gcds2.push(num1.gcd(&num2));
                }
            }
            let end = time::precise_time_s();
            duration2 = end - start;
        }

        assert!(
            gcds.into_iter().map(|i| i.to_bytes_le()).eq(
                gcds2.into_iter().map(|i| i.to_bytes_le())
            )
        );

        println!("n: {:2} =>\t{:.4}s\t\t{:.4}s\t\t{:3.2}", n, duration, duration2, duration / duration2);
    }
}
