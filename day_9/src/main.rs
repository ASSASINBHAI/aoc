use std::fs;
use std::time::Instant;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let disk = input.trim();

    let start = Instant::now();
    part_1(disk);
    println!("Part 1 took: {:.3} sec", start.elapsed().as_secs_f64());

    let start = Instant::now();
    part_2(disk);
    println!("Part 2 took: {:.3} sec", start.elapsed().as_secs_f64());
}

fn part_1(disk: &str) {
    let mut layout = Vec::new();

    // Build the layout from the disk map
    for (idx, ch) in disk.chars().enumerate() {
        let count = ch.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            layout.extend(vec![idx / 2; count]);
        } else {
            layout.extend(vec!['.' as usize; count]);
        }
    }

    // Compact the layout
    while let Some(pos) = layout.iter().position(|&c| c == '.' as usize) {
        let n = layout.pop().unwrap();
        layout[pos] = n;
        while layout.last() == Some(&('.' as usize)) {
            layout.pop();
        }
    }

    // Compute the checksum
    let checksum: usize = layout
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c != '.' as usize)
        .map(|(i, &c)| i * c)
        .sum();

    println!("Part 1 Checksum: {}", checksum);
}

fn part_2(disk: &str) {
    let mut files = Vec::new(); // (id, pos, size)
    let mut free_space = Vec::new(); // (pos, size)

    // Parse the disk into files and free space
    let mut pos = 0;
    for (idx, ch) in disk.chars().enumerate() {
        let size = ch.to_digit(10).unwrap() as usize;

        if idx % 2 == 0 {
            files.push((idx / 2, pos, size));
        } else {
            free_space.push((pos, size));
        }

        pos += size;
    }

    // Compact the files into the free space
    for fidx in (0..files.len()).rev() {
        let (fid, fpos, fsize) = files[fidx];
        if fsize == 0 {
            continue;
        }

        for i in 0..free_space.len() {
            let (free_pos, free_size) = free_space[i];

            if fsize <= free_size && free_pos < fpos {
                // Move file into free space
                files[fidx] = (fid, free_pos, fsize);

                if free_size == fsize {
                    free_space.remove(i);
                } else {
                    free_space[i] = (free_pos + fsize, free_size - fsize);
                }

                free_space.push((fpos, fsize));
                free_space = clean_free_space(&free_space);

                break;
            }
        }
    }

    // Compute the checksum
    let checksum: usize = files
        .iter()
        .flat_map(|&(fid, fpos, fsize)| (0..fsize).map(move |i| (fpos + i) * fid))
        .sum();

    println!("Part 2 Checksum: {}", checksum);
}

fn clean_free_space(free_space: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut new_free_space: Vec<(usize, usize)> = Vec::new();
    let mut sorted_free_space = free_space.to_vec();
    sorted_free_space.sort_by_key(|&(pos, _)| pos);

    for &(fpos, fsize) in &sorted_free_space {
        if fsize == 0 {
            continue;
        }

        if let Some(last) = new_free_space.last_mut() {
            if last.0 + last.1 == fpos {
                last.1 += fsize;
            } else {
                new_free_space.push((fpos, fsize));
            }
        } else {
            new_free_space.push((fpos, fsize));
        }
    }

    new_free_space
}
