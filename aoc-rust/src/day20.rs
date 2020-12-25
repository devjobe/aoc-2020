use std::collections::HashMap;
use std::mem::swap;

fn border_row(tile: &[[char; 10]; 10], i: usize) -> (i32, i32) {
    let mut n = 0;
    let mut m = 0;
    for (ndx, ch) in tile[i].iter().enumerate() {
        let bit = (*ch == '#') as i32;
        n = (n << 1) | bit;
        m = m | (bit << ndx);
    }

    if n > m {
        (m, n)
    } else {
        (n, m)
    }
}

fn border_column(tile: &[[char; 10]; 10], i: usize) -> (i32, i32) {
    let mut n = 0;
    let mut m = 0;
    for (ndx, s) in tile.iter().enumerate() {
        let ch = s[i];
        let bit = (ch == '#') as i32;
        n = (n << 1) | bit;
        m = m | (bit << ndx);
    }

    if n > m {
        (m, n)
    } else {
        (n, m)
    }
}

struct Tile {
    top: (i32, i32),
    bottom: (i32, i32),
    left: (i32, i32),
    right: (i32, i32),
    image: [[char; 10]; 10],
}

fn right_border(tile: &Tile) -> i32 {
    let mut n = 0;
    for row in tile.image.iter() {
        let bit = (row[9] == '#') as i32;
        n = (n << 1) | bit;
    }
    n
}

fn left_border(tile: &Tile) -> i32 {
    let mut n = 0;
    for row in tile.image.iter() {
        let bit = (row[0] == '#') as i32;
        n = (n << 1) | bit;
    }
    n
}

fn bottom_border(tile: &Tile) -> i32 {
    let mut n = 0;
    for &ch in tile.image[9].iter() {
        let bit = (ch == '#') as i32;
        n = (n << 1) | bit;
    }
    n
}

fn top_border(tile: &Tile) -> i32 {
    let mut n = 0;
    for &ch in tile.image[0].iter() {
        let bit = (ch == '#') as i32;
        n = (n << 1) | bit;
    }
    n
}

fn is_connected(border_type: &(i32, i32), borders: &HashMap<(i32, i32), (i32, i32)>) -> bool {
    borders.get(border_type).map_or(false, |(_, b)| *b != 0)
}

fn connected(tile: &Tile, borders: &HashMap<(i32, i32), (i32, i32)>) -> usize {
    let mut c = 0;
    c += (borders.get(&tile.top).unwrap().1 != 0) as usize;
    c += (borders.get(&tile.bottom).unwrap().1 != 0) as usize;
    c += (borders.get(&tile.left).unwrap().1 != 0) as usize;
    c += (borders.get(&tile.right).unwrap().1 != 0) as usize;

    c
}

fn tile_flip_y(tile: &mut Tile) {
    tile.image.reverse();
    swap(&mut tile.top, &mut tile.bottom);
}

fn tile_flip_x(tile: &mut Tile) {
    tile.image.iter_mut().for_each(|x| x.reverse());
    swap(&mut tile.left, &mut tile.right);
}

fn tile_transpose(tile: &mut Tile) {
    let image = &mut tile.image;

    for n in 0..=8 {
        for m in n + 1..=9 {
            let a = image[n][m];
            image[n][m] = image[m][n];
            image[m][n] = a;
        }
    }

    swap(&mut tile.top, &mut tile.left);
    swap(&mut tile.bottom, &mut tile.right);
}

fn make_tile_top_left(tile: &mut Tile, borders: &HashMap<(i32, i32), (i32, i32)>) {
    if !is_connected(&tile.bottom, borders) {
        tile_flip_y(tile);
        if !is_connected(&tile.bottom, borders) {
            panic!("Expected tile to be connected bottom after flip.");
        }
    }

    if !is_connected(&tile.right, borders) {
        tile_flip_x(tile);
        if !is_connected(&tile.right, borders) {
            panic!("Expected tile to be connected right after flip.");
        }
    }

}

fn is_either(nums: &(i32, i32), v: i32) -> bool {
    return nums.0 == v || nums.1 == v;
}

fn make_left_border(tile: &mut Tile, border_type: i32) {
    if is_either(&tile.top, border_type) || is_either(&tile.bottom, border_type) {
        tile_transpose(tile);
    }

    if is_either(&tile.right, border_type) {
        tile_flip_x(tile);
    }

    if left_border(tile) != border_type {
        tile_flip_y(tile);
        if left_border(tile) != border_type {
            panic!("Expected tile to be flipped correctly");
        }
    }
}

fn arrange_tile_right_of(
    tiles: &mut HashMap<i32, Tile>,
    borders: &HashMap<(i32, i32), (i32, i32)>,
    id: i32,
) -> Option<i32> {
    let (border_type, neighbour) = {
        let t = tiles.get(&id).unwrap();
        let (a, b) = borders.get(&t.right).unwrap();
        if *b == 0 {
            return None;
        }
        let n = if *a == id { *b } else { *a };
        (right_border(t), n)
    };

    make_left_border(tiles.get_mut(&neighbour).unwrap(), border_type);

    Some(neighbour)
}

fn make_top_border(tile: &mut Tile, border_type: i32) {
    if is_either(&tile.left, border_type) || is_either(&tile.right, border_type) {
        tile_transpose(tile);
    }

    if is_either(&tile.bottom, border_type) {
        tile_flip_y(tile);
    }

    if top_border(tile) != border_type {
        tile_flip_x(tile);
        if top_border(tile) != border_type {
            panic!("Expected tile to be flipped correctly for top border.");
        }
    }
}

fn arrange_tile_below_of(
    tiles: &mut HashMap<i32, Tile>,
    borders: &HashMap<(i32, i32), (i32, i32)>,
    id: i32,
) -> Option<i32> {
    let (border_type, neighbour) = {
        let t = tiles.get(&id).unwrap();
        let (a, b) = borders.get(&t.bottom).unwrap();
        if *b == 0 {
            return None;
        }
        let n = if *a == id { *b } else { *a };
        (bottom_border(t), n)
    };

    make_top_border(tiles.get_mut(&neighbour).unwrap(), border_type);

    Some(neighbour)
}

fn image_flip_x(image: &mut Vec<Vec<char>>) {
    for row in image {
        row.reverse();
    }
}

fn image_flip_y(image: &mut Vec<Vec<char>>) {
    image.reverse();
}

fn image_transpose(image: &mut Vec<Vec<char>>) {
    let k = image.len();
    for n in 0..=k - 2 {
        for m in n + 1..=k - 1 {
            let a = image[n][m];
            image[n][m] = image[m][n];
            image[m][n] = a;
        }
    }
}

fn match_signature(data: &[char], signature: &str) -> bool {
    for (ch, sig) in data.iter().zip(signature.chars()) {
        if sig == '#' && *ch != '#' {
            return false;
        }
    }
    true
}

fn mark_signature(data: &mut [char], signature: &str) {
    for (ch, sig) in data.iter_mut().zip(signature.chars()) {
        if sig == '#' {
            *ch = 'O';
        }
    }
}

fn mark_sea_monsters(image: &mut Vec<Vec<char>>) -> usize {
    const R0: &'static str = "                  # ";
    const R1: &'static str = "#    ##    ##    ###";
    const R2: &'static str = " #  #  #  #  #  #   ";

    let scan_width = image[0].len() - R0.len();
    let mut monsters = 0;
    for row in 0..image.len() - 3 {
        for col in 0..scan_width {
            if match_signature(&image[row][col..], R0)
                && match_signature(&image[row + 1][col..], R1)
                && match_signature(&image[row + 2][col..], R2)
            {
                mark_signature(&mut image[row][col..], R0);
                mark_signature(&mut image[row + 1][col..], R1);
                mark_signature(&mut image[row + 2][col..], R2);
                monsters += 1;
            }
        }
    }
    monsters
}

pub fn run() {
    let input = std::fs::read_to_string("../input/day20.txt").unwrap();

    let mut tiles = HashMap::new();
    let mut borders = HashMap::new();

    for tile in input.split("\r\n\r\n") {
        let id = tile[5..9].parse::<i32>().unwrap();

        let mut image = [['.'; 10]; 10];

        for i in 0..10 {
            for (a, b) in image[i]
                .iter_mut()
                .zip(tile[(i + 1) * 12..(i + 1) * 12 + 10].chars())
            {
                *a = b;
            }
        }

        let top = border_row(&image, 0);
        let bottom = border_row(&image, 9);
        let left = border_column(&image, 0);
        let right = border_column(&image, 9);

        tiles.insert(
            id,
            Tile {
                top,
                bottom,
                left,
                right,
                image,
            },
        );

        borders
            .entry(top)
            .and_modify(|(_, b)| {
                if *b != 0 {
                    panic!("Expected b == 0");
                }
                *b = id;
            })
            .or_insert((id, 0));
        borders
            .entry(bottom)
            .and_modify(|(_, b)| {
                if *b != 0 {
                    panic!("Expected b == 0");
                }
                *b = id;
            })
            .or_insert((id, 0));
        borders
            .entry(left)
            .and_modify(|(_, b)| {
                if *b != 0 {
                    panic!("Expected b == 0");
                }
                *b = id;
            })
            .or_insert((id, 0));
        borders
            .entry(right)
            .and_modify(|(_, b)| {
                if *b != 0 {
                    panic!("Expected b == 0");
                }
                *b = id;
            })
            .or_insert((id, 0));
    }

    let mut res = 1;
    for (id, tile) in &tiles {
        if connected(tile, &borders) == 2 {
            res *= *id as i64;
        }
    }

    println!("Day 20.a: {}", res);

    let mut left_id = {
        let (id, tile) = tiles
            .iter_mut()
            .find(|(_, tile)| connected(tile, &borders) == 2)
            .unwrap();
        make_tile_top_left(tile, &borders);
        *id
    };

    let mut row_start = left_id;
    let mut layout = Vec::with_capacity(tiles.len());
    let mut row_length = None;
    loop {
        layout.push(left_id);
        if let Some(id) = arrange_tile_right_of(&mut tiles, &borders, left_id) {
            left_id = id;
        } else if let Some(id) = arrange_tile_below_of(&mut tiles, &borders, row_start) {
            if row_length == None {
                row_length = Some(layout.len());
            }
            left_id = id;
            row_start = id;
        } else {
            break;
        }
    }

    let row_length = row_length.unwrap_or(layout.len());
    let rows = (layout.len() / row_length) * 8;
    let cols = row_length * 8;
    let mut image = vec![vec![' '; cols]; rows];

    for (chunk_row, row) in layout.chunks(row_length).enumerate() {
        for (chunk_col, col) in row.iter().enumerate() {
            let part = &tiles.get(col).unwrap().image;
            for (data, image_row) in part[1..=8].iter().zip(&mut image[chunk_row * 8..]) {
                for (v, ch) in data[1..=8].iter().zip(&mut image_row[chunk_col * 8..]) {
                    *ch = *v;
                }
            }
        }
    }

    loop {
        let mut n = mark_sea_monsters(&mut image);
        if n == 0 {
            image_flip_x(&mut image);
            n = mark_sea_monsters(&mut image);
            if n == 0 {
                image_flip_y(&mut image);
                n = mark_sea_monsters(&mut image);
                if n == 0 {
                    image_flip_x(&mut image);
                    n = mark_sea_monsters(&mut image);
                }
            }
        }

        if n != 0 {
            break;
        }

        image_transpose(&mut image);
    }

    println!(
        "Day 20.b: {}",
        image
            .iter()
            .map(|row| row.iter().filter(|&&x| x == '#').count())
            .sum::<usize>()
    );
}
