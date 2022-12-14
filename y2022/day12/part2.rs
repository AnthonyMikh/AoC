#[derive(Clone, Copy)]
pub struct Bounds {
    rows: usize,
    cols: usize,
}

type Pos = (usize, usize);

impl Bounds {
    pub fn around(self, buf: &mut [Pos; 4], (row, col): Pos) -> &[Pos] {
        let mut i = 0;
        let mut put = |r, c| {
            buf[i] = (r, c);
            i += 1;
        };
        if let Some(row) = row.checked_sub(1) {
            put(row, col);
        }
        if let Some(col) = col.checked_sub(1) {
            put(row, col);
        }
        if col + 1 < self.cols {
            put(row, col + 1);
        }
        if row + 1 < self.rows {
            put(row + 1, col);
        }
        &buf[..i]
    }
}

struct Grid {
    cells: Vec<Vec<u8>>,
    bounds: Bounds,
    starts: Vec<Pos>,
    target: Pos,
}

impl Grid {
    fn parse(s: &str) -> Self {
        let mut cells = s.lines().map(|l| l.as_bytes().to_vec()).collect::<Vec<_>>();
        let bounds = Bounds {
            rows: cells.len(),
            cols: cells[0].len(),
        };
        let mut starts = Vec::new();
        let mut target = None;

        for (irow, row) in cells.iter_mut().enumerate() {
            for (icol, ch) in row.iter_mut().enumerate() {
                match ch {
                    b'a' | b'S' => {
                        starts.push((irow, icol));
                        *ch = b'a';
                    }
                    b'E' => {
                        target = Some((irow, icol));
                        *ch = b'z';
                    }
                    _ => (),
                }
            }
        }

        Self {
            cells,
            bounds,
            starts,
            target: target.unwrap(),
        }
    }
}

fn solve(input: &str) -> usize {
    const BLANK: [Pos; 4] = [(0, 0); 4];

    let Grid {
        cells,
        bounds,
        starts,
        target,
    } = Grid::parse(input);
    let mut visited = vec![vec![false; bounds.cols]; bounds.rows];
    let mut queue = starts
        .into_iter()
        .map(|pos| (pos, 0))
        .collect::<std::collections::VecDeque<_>>();

    while let Some((pos, len)) = queue.pop_front() {
        let (row, col) = pos;
        if visited[row][col] {
            continue;
        }
        visited[row][col] = true;
        let len = len + 1;
        let height = cells[row][col];

        #[allow(const_item_mutation)]
        for &next @ (row, col) in bounds.around(&mut BLANK, pos) {
            if visited[next.0][next.1] {
                continue;
            }
            if cells[row][col] > height + 1 {
                continue;
            }
            if next == target {
                return len;
            }
            queue.push_back((next, len));
        }
    }

    unreachable!()
}

fn main() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    println!("{}", solve(input));
}
