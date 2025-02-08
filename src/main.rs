use itertools::Itertools;
use thin_vec::ThinVec;
use std::ops::Range;

fn main() {
    let mut squares_by_residue_class = [vec![], vec![], vec![]];
    let mut sums_by_complement_class = [vec![], vec![], vec![]];
    let mut magic_sums_just_finished = 3..3;

    for number in 1_u32.. {
        let square = number as u64 * number as u64;
        if square % 24 != 1 { continue; }

        let residue = square % 72;
        let residue_class = (residue / 24) as usize;

        magic_sums_just_finished.end = square + 1 + 1;
        while magic_sums_just_finished.end % 72 != 3 { magic_sums_just_finished.end += 24; }

        let magic_sums0 = magic_sums(magic_sums_just_finished.clone(), &squares_by_residue_class[0], &sums_by_complement_class[0]);
        let magic_sums1 = magic_sums(magic_sums_just_finished.clone(), &squares_by_residue_class[1], &sums_by_complement_class[1]);
        let magic_sums2 = magic_sums(magic_sums_just_finished.clone(), &squares_by_residue_class[2], &sums_by_complement_class[2]);

        let magic_sums = [magic_sums0, magic_sums1, magic_sums2].into_iter().kmerge_by(|a, b| a.0 < b.0);
        for (magic_sum, iterator) in magic_sums {
            for (square1, numbers) in iterator {
                let square_sum = magic_sum - square1;

                for &number in numbers {
                    let square2 = number as u64 * number as u64;
                    let _square3 = square_sum - square2;
                    //println!("{} = {} + {} + {}", magic_sum, square1, square2, square3);
                }
            }
        }

        let max_sum = (square * 2) as usize;
        sums_by_complement_class.iter_mut().for_each(|vec| vec.resize(max_sum / 72 + 1, ThinVec::new()));

        for (i, squares) in squares_by_residue_class.iter().enumerate() {
            let complement_class = (6 - residue_class - i) % 3;
            let sums_for_complement = &mut sums_by_complement_class[complement_class];

            for &square2 in squares {
                let square_sum = square + square2;
                sums_for_complement[square_sum as usize / 72].push(number);
            }
        }

        squares_by_residue_class[residue_class].push(square);
        magic_sums_just_finished.start = magic_sums_just_finished.end;

        if number > 7000 { break; }
    }
}

fn magic_sums<'a>(just_finished: Range<u64>, squares: &[u64], sums: &'a [ThinVec<u32>]) -> impl Iterator<Item=(u64, impl Iterator<Item=(u64, &'a [u32])>)> {
    just_finished.step_by(72).map(move |magic_sum| {
        let minimum_square = (magic_sum as f64 / 3.) as u64;
        let minimum_index = squares.partition_point(|&s| s < minimum_square);

        (magic_sum, squares[minimum_index..].iter().map(move |&square| {
            let square_sum = magic_sum - square;
            let numbers = &sums[square_sum as usize / 72];
            let partition = numbers.partition_point(|&n| (n as u64 * n as u64) < square);

            (square, &numbers[0..partition])
        }))
    })
}
