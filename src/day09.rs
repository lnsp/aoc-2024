use std::cmp::min;

fn checksum(filesystem: &Vec<(i32, usize)>) -> usize {
    filesystem
        .iter()
        .fold((0, 0), |(index, sum), entry| {
            if entry.0 == -1 {
                (index + entry.1, sum)
            } else {
                (
                    index + entry.1,
                    sum + (index..(index + entry.1))
                        .map(|v| v * entry.0 as usize)
                        .sum::<usize>(),
                )
            }
        })
        .1
}

pub fn task1_better(mut filesystem: Vec<(i32, usize)>) -> usize {
    let mut left = 0;
    let mut right = filesystem.len() - 1;
    loop {
        while left < right && filesystem[left].0 != -1 {
            left += 1;
        }
        while right > left && filesystem[right].0 == -1 {
            right -= 1;
        }
        if left == right || filesystem[left].0 != -1 || filesystem[right].0 == -1 {
            return checksum(&filesystem)
        }

        let block = filesystem[right].0;
        let diff = min(filesystem[left].1, filesystem[right].1);

        filesystem[left].1 -= diff;
        filesystem[right].1 -= diff;

        if filesystem[left].1 > 0 {
            filesystem.insert(left, (block, diff));
            right += 1;
        } else {
            filesystem[left] = (filesystem[right].0, diff);
        }

        if filesystem[right].1 > 0 {
            filesystem.insert(right, (-1, diff));
            right += 1;
        } else {
            filesystem[right] = (-1, diff);
        }
    }
}

pub fn task1(filesystem: &Vec<i32>) -> usize {
    let mut filesystem_clone = filesystem.clone();
    let (mut left_ptr, mut right_ptr) = (0, filesystem.len() - 1);
    loop {
        while left_ptr < right_ptr && filesystem_clone[left_ptr] != -1 {
            left_ptr += 1
        }
        while right_ptr > left_ptr && filesystem_clone[right_ptr] == -1 {
            right_ptr -= 1
        }
        if left_ptr == right_ptr
            || filesystem_clone[left_ptr] != -1
            || filesystem_clone[right_ptr] == -1
        {
            println!("{:?}", filesystem_clone);
            return filesystem_clone
                .iter()
                .enumerate()
                .filter(|&(_, block)| *block != -1)
                .map(|(index, block)| index * *block as usize)
                .sum();
        }
        println!("{} {}", left_ptr, right_ptr);
        (filesystem_clone[left_ptr], filesystem_clone[right_ptr]) =
            (filesystem_clone[right_ptr], filesystem_clone[left_ptr]);
    }
}

pub fn task2(mut filesystem: Vec<(i32, usize)>) -> usize {
    let max_block = filesystem.iter().map(|&(block, _)| block).max().unwrap();
    for block in (0..=max_block).rev() {
        // Find block index
        let block_pos = filesystem
            .iter()
            .enumerate()
            .filter(|&(_, b)| b.0 == block)
            .map(|(index, _)| index)
            .next()
            .unwrap();
        let block_len = filesystem[block_pos].1;
        // Find free space
        if let Some(swap_pos) = filesystem
            .iter()
            .enumerate()
            .filter(|&(index, b)| index < block_pos && b.0 == -1 && b.1 >= block_len)
            .map(|(index, _)| index)
            .next()
        {
            if block_pos > 0 && filesystem[block_pos-1].0 == -1 {
                filesystem[block_pos-1].1 += block_len;
                filesystem.remove(block_pos);
            } else {
                filesystem[block_pos] = (-1, block_len);
            }
            if filesystem[swap_pos].1 > block_len {
                filesystem[swap_pos].1 -= block_len;
                filesystem.insert(swap_pos, (block, block_len));
            } else {
                filesystem[swap_pos] = (block, block_len);
            }
        }
    }

    checksum(&filesystem)
}
