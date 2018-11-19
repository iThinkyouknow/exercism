use std::iter;
#[derive(Debug, Clone)]
enum Direction {
    Left,
    Down,
    Right,
    Up,
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // let size_u = size as usize;
    // let mut nums = (1..=(size.pow(2))).peekable();
    // let mut curr_dir = Direction::Right;
    // let mut steps_in_direction = size_u;
    // let mut yx: (usize, usize) = (0, 0);

    // let mut matrix: Vec<Vec<u32>> = vec![vec![0; size_u]; size_u];

    // while steps_in_direction > 0 {
    //     nums.by_ref()
    //         .take(steps_in_direction)
    //         .enumerate()
    //         .for_each(|(index, d)| {
    //             matrix[yx.0][yx.1] = d;
    //             match curr_dir {
    //                 Direction::Right if index == steps_in_direction - 1 => {
    //                     curr_dir = Direction::Down;
    //                     steps_in_direction -= 1;
    //                     yx = (yx.0 + 1, yx.1)
    //                 }
    //                 Direction::Right => yx = (yx.0, yx.1 + 1),
    //                 Direction::Down if index == steps_in_direction - 1 => {
    //                     curr_dir = Direction::Left;
    //                     yx = (yx.0, yx.1 - 1)
    //                 }
    //                 Direction::Down => yx = (yx.0 + 1, yx.1),
    //                 Direction::Left if index == steps_in_direction - 1 => {
    //                     curr_dir = Direction::Up;
    //                     steps_in_direction -= 1;
    //                     yx = (yx.0 - 1, yx.1)
    //                 }
    //                 Direction::Left => yx = (yx.0, yx.1 - 1),
    //                 Direction::Up if index == steps_in_direction - 1 => {
    //                     curr_dir = Direction::Right;
    //                     yx = (yx.0, yx.1 + 1)
    //                 }
    //                 Direction::Up => yx = (yx.0 - 1, yx.1),
    //             };
    //         });
    // }

    // matrix
    let size_u = size as usize;
    let matrix: Vec<Vec<u32>> = vec![vec![0; size_u]; size_u];

    let mut directions_iter = [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ].into_iter()
        .cycle();

    (1..=size_u)
        .rev()
        .enumerate()
        .flat_map(|(i, steps)| {
            let times = match i {
                0 => 1,
                _ => 2,
            };
            iter::repeat(steps).take(times).collect::<Vec<usize>>()
        })
        .flat_map(|times| {
            iter::repeat(directions_iter.next().unwrap().clone())
                .take(times)
                .collect::<Vec<Direction>>()
        })
        .enumerate()
        .scan((0usize, 0usize), |coords, (i, direction)| {
            match i == 0 {
                true => (),
                false => {
                    let (y, x) = *coords;
                    *coords = match direction {
                        Direction::Right => (y , x + 1),
                        Direction::Down => (y + 1, x),
                        Direction::Left => (y, x - 1),
                        Direction::Up => (y - 1, x),
                    };
                }
            };
            Some(*coords)
        })
        .enumerate()
        .fold(matrix, |mut mat, (i, coords)| {
            let (y, x) = coords;

            mat[y][x] = i as u32 + 1;
            mat
        })
}
