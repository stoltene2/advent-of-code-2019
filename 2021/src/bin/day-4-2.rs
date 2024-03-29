use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
struct Line([i32; 5]);

#[derive(Debug)]
struct MarkedLine(Vec<i32>);

#[derive(Debug)]
struct Board {
    rows: [Line; 5],
    cols: [Line; 5],
    row_marks: [MarkedLine; 5],
    col_marks: [MarkedLine; 5],
}

impl Line {
    fn has_number(&self, n: i32) -> bool {
        self.0.iter().any(|m| n == *m)
    }

    fn sum(&self) -> i32 {
        self.0.iter().sum()
    }
}

impl Index<usize> for Line {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        &(self.0)[index]
    }
}

impl IndexMut<usize> for Line {
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        &mut (self.0)[index]
    }
}

impl MarkedLine {
    fn is_winner(&self) -> bool {
        self.0.len() == 5
    }

    fn add_mark(&mut self, n: i32) {
        self.0.push(n);
    }

    fn sum(&self) -> i32 {
        self.0.iter().sum()
    }

    fn new() -> MarkedLine {
        MarkedLine(Vec::with_capacity(5))
    }
}

impl Board {
    pub fn new(data: &Vec<Vec<i32>>) -> Board {
        let mut rows: [Line; 5] = [
            Line([0, 0, 0, 0, 0]),
            Line([0, 0, 0, 0, 0]),
            Line([0, 0, 0, 0, 0]),
            Line([0, 0, 0, 0, 0]),
            Line([0, 0, 0, 0, 0]),
        ];

        let mut cols: [Line; 5] = rows.clone();

        for (i, row) in data.iter().enumerate() {
            for (j, elt) in row.iter().enumerate() {
                rows[i][j] = *elt;
                cols[j][i] = *elt;
            }
        }

        Board {
            rows: rows,
            cols: cols,
            row_marks: [
                MarkedLine::new(),
                MarkedLine::new(),
                MarkedLine::new(),
                MarkedLine::new(),
                MarkedLine::new(),
            ],
            col_marks: [
                MarkedLine::new(),
                MarkedLine::new(),
                MarkedLine::new(),
                MarkedLine::new(),
                MarkedLine::new(),
            ],
        }
    }

    pub fn mark(self: &mut Self, n: i32) {
        for (i, row) in self.rows.iter().enumerate() {
            if row.has_number(n) {
                self.row_marks[i].add_mark(n);
            }
        }

        for (j, col) in self.cols.iter().enumerate() {
            if col.has_number(n) {
                self.col_marks[j].add_mark(n);
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        self.row_marks.iter().any(|r| r.is_winner()) || self.col_marks.iter().any(|r| r.is_winner())
    }

    pub fn unmarked_sum(&self) -> i32 {
        let board_sum = self.rows.iter().fold(0, |total, row| total + row.sum());
        let marked_sum = self
            .row_marks
            .iter()
            .fold(0, |total, row| total + row.sum());
        board_sum - marked_sum
    }
}

fn main() {
    let test_data: Vec<Vec<i32>> = vec![
        vec![3, 15, 0, 2, 22],
        vec![9, 18, 13, 17, 5],
        vec![19, 8, 7, 25, 23],
        vec![20, 11, 10, 24, 4],
        vec![14, 21, 16, 12, 6],
    ];

    let mut board: Board = Board::new(&test_data);

    let calls = vec![
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    for i in calls {
        board.mark(i);

        if board.is_winner() {
            let result = board.unmarked_sum() * i;
            println!("p2 example winner: {}, {}", i, result);
            assert_eq!(1924, result);
            break;
        }
    }

    assert!(board.is_winner());

    let calls = vec![
        42, 44, 71, 26, 70, 92, 77, 45, 6, 18, 79, 54, 31, 34, 64, 32, 16, 55, 81, 11, 90, 10, 21,
        87, 0, 84, 8, 23, 1, 12, 60, 20, 57, 68, 61, 82, 49, 59, 22, 2, 63, 33, 50, 39, 28, 30, 88,
        41, 69, 72, 98, 73, 7, 65, 53, 35, 96, 67, 36, 4, 51, 75, 24, 86, 97, 85, 66, 29, 74, 40,
        93, 58, 9, 62, 95, 91, 80, 99, 14, 19, 43, 37, 27, 56, 94, 25, 83, 48, 17, 38, 78, 15, 52,
        76, 5, 13, 46, 89, 47, 3,
    ];
    let mut boards: Vec<Board> = input().iter().map(|data| Board::new(data)).collect();

    for i in calls {
        boards.iter_mut().for_each(|board| {
            board.mark(i);
        });

        if boards.len() == 1 && boards[0].is_winner() {
            // 7700 is too high
            println!("Last winning board: {}", boards[0].unmarked_sum() * i);
            break;
        }

        boards = boards.into_iter().filter(|b| !b.is_winner()).collect();
    }
}

fn input() -> Vec<Vec<Vec<i32>>> {
    vec![
        vec![
            vec![48, 69, 68, 49, 13],
            vec![25, 14, 30, 74, 89],
            vec![16, 38, 19, 24, 29],
            vec![56, 97, 50, 65, 79],
            vec![57, 52, 5, 27, 76],
        ],
        vec![
            vec![65, 69, 73, 60, 66],
            vec![79, 23, 95, 32, 56],
            vec![36, 51, 26, 1, 28],
            vec![76, 9, 3, 71, 77],
            vec![41, 15, 61, 68, 14],
        ],
        vec![
            vec![77, 86, 11, 96, 50],
            vec![64, 35, 76, 14, 5],
            vec![31, 20, 53, 84, 66],
            vec![83, 40, 1, 28, 79],
            vec![15, 74, 92, 65, 39],
        ],
        vec![
            vec![20, 55, 65, 13, 48],
            vec![1, 3, 98, 78, 29],
            vec![58, 45, 39, 18, 34],
            vec![43, 26, 83, 33, 50],
            vec![84, 15, 62, 71, 88],
        ],
        vec![
            vec![0, 10, 3, 19, 47],
            vec![51, 43, 62, 11, 70],
            vec![38, 59, 56, 81, 55],
            vec![77, 36, 39, 91, 74],
            vec![23, 14, 67, 12, 80],
        ],
        vec![
            vec![36, 18, 47, 14, 60],
            vec![33, 96, 84, 44, 72],
            vec![1, 37, 74, 93, 80],
            vec![50, 95, 59, 90, 27],
            vec![63, 53, 65, 41, 78],
        ],
        vec![
            vec![31, 4, 85, 55, 74],
            vec![65, 73, 2, 52, 0],
            vec![99, 82, 83, 25, 81],
            vec![70, 75, 90, 91, 89],
            vec![44, 15, 53, 67, 39],
        ],
        vec![
            vec![11, 39, 36, 37, 32],
            vec![84, 19, 58, 34, 48],
            vec![3, 69, 81, 41, 59],
            vec![86, 2, 56, 47, 90],
            vec![31, 12, 17, 14, 40],
        ],
        vec![
            vec![28, 13, 34, 45, 37],
            vec![83, 55, 61, 80, 92],
            vec![77, 33, 44, 40, 31],
            vec![54, 25, 71, 56, 93],
            vec![60, 70, 20, 65, 9],
        ],
        vec![
            vec![82, 60, 11, 40, 90],
            vec![0, 75, 86, 55, 58],
            vec![24, 9, 72, 89, 56],
            vec![54, 16, 22, 70, 57],
            vec![64, 39, 31, 33, 5],
        ],
        vec![
            vec![56, 24, 42, 76, 29],
            vec![0, 60, 12, 84, 73],
            vec![72, 81, 99, 15, 70],
            vec![88, 90, 80, 26, 65],
            vec![61, 47, 19, 7, 17],
        ],
        vec![
            vec![28, 64, 80, 3, 41],
            vec![32, 0, 22, 18, 97],
            vec![87, 94, 1, 52, 83],
            vec![37, 29, 14, 5, 82],
            vec![73, 58, 56, 88, 49],
        ],
        vec![
            vec![70, 23, 43, 81, 66],
            vec![27, 14, 38, 8, 13],
            vec![26, 17, 6, 67, 83],
            vec![91, 96, 37, 39, 18],
            vec![19, 25, 77, 98, 53],
        ],
        vec![
            vec![33, 12, 42, 4, 44],
            vec![88, 99, 61, 27, 43],
            vec![68, 13, 38, 57, 87],
            vec![59, 20, 2, 90, 40],
            vec![80, 64, 77, 94, 21],
        ],
        vec![
            vec![47, 29, 58, 72, 24],
            vec![26, 66, 42, 68, 36],
            vec![34, 80, 76, 94, 59],
            vec![61, 41, 64, 44, 50],
            vec![2, 28, 20, 9, 17],
        ],
        vec![
            vec![88, 3, 36, 39, 0],
            vec![83, 54, 86, 9, 92],
            vec![14, 82, 34, 62, 73],
            vec![63, 95, 78, 23, 55],
            vec![24, 41, 47, 2, 58],
        ],
        vec![
            vec![78, 85, 93, 38, 30],
            vec![41, 0, 29, 39, 40],
            vec![28, 76, 98, 60, 96],
            vec![26, 94, 35, 1, 82],
            vec![66, 56, 31, 64, 45],
        ],
        vec![
            vec![30, 15, 12, 44, 24],
            vec![32, 49, 99, 76, 8],
            vec![64, 56, 43, 42, 19],
            vec![62, 83, 33, 48, 54],
            vec![89, 74, 72, 3, 91],
        ],
        vec![
            vec![76, 48, 51, 11, 16],
            vec![67, 78, 71, 62, 58],
            vec![56, 29, 74, 5, 17],
            vec![9, 81, 65, 12, 39],
            vec![19, 24, 1, 13, 30],
        ],
        vec![
            vec![72, 96, 63, 4, 62],
            vec![71, 47, 9, 56, 90],
            vec![12, 86, 52, 10, 3],
            vec![49, 83, 73, 7, 87],
            vec![85, 64, 48, 81, 32],
        ],
        vec![
            vec![28, 72, 38, 47, 63],
            vec![37, 14, 13, 51, 27],
            vec![8, 82, 46, 86, 55],
            vec![22, 18, 59, 81, 10],
            vec![77, 21, 58, 24, 25],
        ],
        vec![
            vec![92, 49, 67, 19, 85],
            vec![45, 50, 41, 79, 84],
            vec![4, 0, 75, 17, 53],
            vec![91, 23, 28, 82, 6],
            vec![60, 37, 18, 13, 69],
        ],
        vec![
            vec![1, 27, 74, 89, 4],
            vec![29, 9, 78, 48, 54],
            vec![10, 38, 13, 40, 49],
            vec![71, 73, 79, 77, 17],
            vec![25, 22, 91, 20, 2],
        ],
        vec![
            vec![94, 40, 75, 63, 10],
            vec![89, 81, 32, 72, 73],
            vec![8, 65, 68, 62, 58],
            vec![76, 97, 57, 34, 66],
            vec![0, 82, 14, 90, 11],
        ],
        vec![
            vec![30, 89, 82, 29, 85],
            vec![41, 78, 91, 73, 14],
            vec![66, 98, 21, 23, 16],
            vec![63, 65, 99, 10, 58],
            vec![67, 51, 17, 11, 19],
        ],
        vec![
            vec![37, 74, 52, 31, 28],
            vec![48, 14, 13, 72, 59],
            vec![45, 30, 46, 7, 9],
            vec![89, 77, 16, 33, 81],
            vec![62, 4, 15, 5, 96],
        ],
        vec![
            vec![72, 63, 77, 57, 86],
            vec![8, 65, 11, 44, 69],
            vec![78, 52, 14, 84, 80],
            vec![36, 42, 30, 59, 33],
            vec![46, 10, 92, 19, 26],
        ],
        vec![
            vec![13, 65, 92, 1, 5],
            vec![71, 54, 28, 33, 98],
            vec![42, 94, 30, 35, 78],
            vec![57, 85, 25, 60, 7],
            vec![43, 80, 48, 97, 27],
        ],
        vec![
            vec![29, 64, 61, 99, 74],
            vec![94, 68, 72, 36, 51],
            vec![11, 83, 21, 96, 86],
            vec![56, 53, 25, 48, 92],
            vec![60, 15, 19, 50, 76],
        ],
        vec![
            vec![12, 61, 89, 99, 30],
            vec![84, 50, 34, 74, 55],
            vec![26, 72, 36, 86, 3],
            vec![4, 79, 6, 8, 40],
            vec![5, 83, 82, 66, 44],
        ],
        vec![
            vec![40, 93, 61, 20, 50],
            vec![90, 7, 60, 38, 16],
            vec![22, 52, 3, 92, 5],
            vec![32, 0, 57, 58, 30],
            vec![68, 28, 81, 46, 54],
        ],
        vec![
            vec![86, 72, 55, 48, 24],
            vec![21, 3, 25, 22, 20],
            vec![16, 64, 97, 77, 68],
            vec![66, 2, 9, 47, 30],
            vec![36, 87, 67, 23, 0],
        ],
        vec![
            vec![18, 54, 51, 38, 44],
            vec![73, 50, 68, 59, 61],
            vec![45, 0, 5, 80, 19],
            vec![79, 65, 66, 30, 15],
            vec![26, 74, 23, 99, 95],
        ],
        vec![
            vec![75, 9, 34, 74, 86],
            vec![53, 14, 25, 61, 31],
            vec![17, 60, 10, 32, 93],
            vec![47, 33, 0, 77, 68],
            vec![38, 45, 4, 55, 39],
        ],
        vec![
            vec![32, 81, 21, 11, 31],
            vec![30, 56, 3, 25, 18],
            vec![80, 96, 20, 65, 71],
            vec![41, 67, 22, 97, 10],
            vec![93, 68, 17, 13, 1],
        ],
        vec![
            vec![54, 42, 23, 8, 83],
            vec![87, 63, 36, 95, 81],
            vec![40, 14, 13, 6, 18],
            vec![44, 16, 89, 59, 69],
            vec![17, 10, 73, 76, 5],
        ],
        vec![
            vec![36, 69, 2, 13, 74],
            vec![17, 7, 9, 42, 54],
            vec![79, 21, 5, 19, 66],
            vec![68, 22, 4, 85, 25],
            vec![63, 23, 10, 75, 87],
        ],
        vec![
            vec![27, 61, 41, 78, 4],
            vec![77, 76, 82, 85, 17],
            vec![74, 0, 54, 63, 34],
            vec![6, 56, 1, 16, 89],
            vec![8, 12, 36, 59, 81],
        ],
        vec![
            vec![37, 22, 9, 36, 19],
            vec![79, 61, 78, 96, 91],
            vec![41, 11, 42, 8, 24],
            vec![85, 84, 35, 47, 31],
            vec![50, 16, 62, 53, 21],
        ],
        vec![
            vec![93, 61, 14, 78, 75],
            vec![51, 88, 3, 54, 37],
            vec![57, 36, 23, 77, 91],
            vec![72, 47, 39, 65, 35],
            vec![2, 58, 86, 81, 27],
        ],
        vec![
            vec![0, 34, 82, 13, 53],
            vec![5, 88, 42, 11, 45],
            vec![26, 81, 28, 46, 24],
            vec![67, 71, 95, 6, 9],
            vec![69, 97, 41, 15, 7],
        ],
        vec![
            vec![92, 25, 31, 62, 35],
            vec![83, 72, 54, 61, 41],
            vec![28, 51, 45, 79, 3],
            vec![16, 38, 52, 58, 2],
            vec![37, 7, 36, 21, 22],
        ],
        vec![
            vec![92, 33, 21, 47, 89],
            vec![14, 40, 23, 82, 59],
            vec![42, 73, 4, 94, 72],
            vec![67, 63, 16, 8, 75],
            vec![70, 43, 48, 81, 13],
        ],
        vec![
            vec![63, 99, 85, 39, 15],
            vec![58, 68, 62, 50, 24],
            vec![25, 60, 26, 5, 94],
            vec![8, 53, 75, 46, 61],
            vec![66, 3, 16, 47, 42],
        ],
        vec![
            vec![21, 89, 33, 61, 12],
            vec![36, 83, 76, 80, 24],
            vec![93, 95, 4, 0, 7],
            vec![91, 43, 17, 14, 64],
            vec![84, 71, 45, 44, 5],
        ],
        vec![
            vec![23, 74, 19, 40, 42],
            vec![83, 75, 15, 12, 21],
            vec![4, 11, 90, 51, 53],
            vec![1, 77, 54, 62, 82],
            vec![28, 5, 46, 52, 79],
        ],
        vec![
            vec![73, 60, 22, 3, 71],
            vec![53, 67, 30, 58, 15],
            vec![59, 89, 40, 21, 31],
            vec![13, 42, 84, 97, 10],
            vec![81, 6, 70, 88, 52],
        ],
        vec![
            vec![91, 26, 38, 80, 79],
            vec![46, 24, 66, 53, 93],
            vec![25, 2, 58, 59, 63],
            vec![65, 78, 36, 7, 1],
            vec![83, 72, 50, 39, 21],
        ],
        vec![
            vec![31, 22, 32, 36, 79],
            vec![97, 60, 62, 43, 96],
            vec![7, 15, 71, 89, 87],
            vec![76, 95, 10, 19, 48],
            vec![68, 69, 29, 24, 56],
        ],
        vec![
            vec![25, 40, 47, 50, 31],
            vec![15, 13, 45, 66, 30],
            vec![12, 67, 95, 3, 48],
            vec![43, 61, 60, 75, 59],
            vec![16, 69, 98, 6, 88],
        ],
        vec![
            vec![10, 80, 15, 17, 23],
            vec![87, 52, 56, 51, 22],
            vec![43, 58, 96, 63, 27],
            vec![29, 13, 33, 66, 25],
            vec![16, 97, 88, 90, 77],
        ],
        vec![
            vec![5, 11, 90, 8, 18],
            vec![51, 42, 73, 25, 85],
            vec![69, 94, 79, 53, 32],
            vec![72, 23, 57, 15, 3],
            vec![78, 28, 47, 37, 35],
        ],
        vec![
            vec![80, 81, 44, 53, 14],
            vec![36, 71, 35, 83, 30],
            vec![94, 40, 2, 99, 97],
            vec![16, 48, 85, 76, 20],
            vec![56, 25, 89, 88, 39],
        ],
        vec![
            vec![34, 54, 9, 36, 44],
            vec![14, 19, 0, 64, 40],
            vec![25, 78, 74, 18, 13],
            vec![82, 10, 6, 92, 95],
            vec![84, 8, 75, 98, 45],
        ],
        vec![
            vec![95, 8, 27, 29, 89],
            vec![33, 79, 88, 59, 24],
            vec![4, 28, 35, 72, 97],
            vec![22, 77, 85, 94, 76],
            vec![46, 43, 68, 65, 67],
        ],
        vec![
            vec![42, 13, 3, 20, 43],
            vec![12, 98, 31, 69, 4],
            vec![56, 67, 25, 89, 71],
            vec![16, 29, 33, 1, 36],
            vec![27, 80, 0, 47, 5],
        ],
        vec![
            vec![62, 66, 86, 63, 70],
            vec![44, 27, 5, 78, 85],
            vec![49, 40, 58, 61, 89],
            vec![37, 43, 36, 68, 28],
            vec![22, 7, 35, 52, 57],
        ],
        vec![
            vec![73, 62, 90, 75, 81],
            vec![31, 94, 77, 44, 14],
            vec![18, 9, 10, 93, 17],
            vec![89, 39, 63, 66, 53],
            vec![42, 88, 59, 36, 2],
        ],
        vec![
            vec![54, 52, 76, 79, 61],
            vec![95, 39, 3, 41, 63],
            vec![74, 97, 38, 0, 15],
            vec![6, 23, 11, 10, 83],
            vec![46, 67, 96, 77, 29],
        ],
        vec![
            vec![0, 76, 9, 84, 52],
            vec![45, 27, 29, 56, 85],
            vec![89, 63, 77, 49, 65],
            vec![4, 13, 78, 33, 37],
            vec![64, 7, 1, 94, 12],
        ],
        vec![
            vec![16, 32, 78, 73, 90],
            vec![2, 48, 57, 75, 23],
            vec![95, 27, 33, 12, 51],
            vec![69, 26, 17, 42, 61],
            vec![67, 89, 74, 44, 15],
        ],
        vec![
            vec![47, 90, 5, 76, 96],
            vec![71, 24, 9, 69, 16],
            vec![64, 23, 98, 66, 81],
            vec![29, 34, 63, 72, 22],
            vec![41, 17, 45, 87, 57],
        ],
        vec![
            vec![48, 45, 87, 14, 1],
            vec![86, 28, 6, 62, 46],
            vec![27, 92, 11, 49, 94],
            vec![0, 90, 10, 70, 20],
            vec![2, 59, 88, 96, 33],
        ],
        vec![
            vec![16, 29, 68, 4, 82],
            vec![56, 67, 2, 69, 25],
            vec![94, 47, 61, 51, 32],
            vec![70, 31, 21, 43, 42],
            vec![76, 60, 17, 59, 92],
        ],
        vec![
            vec![18, 6, 87, 56, 61],
            vec![89, 53, 5, 47, 69],
            vec![93, 57, 78, 41, 82],
            vec![27, 24, 51, 99, 29],
            vec![42, 74, 28, 75, 97],
        ],
        vec![
            vec![21, 62, 60, 86, 85],
            vec![24, 35, 1, 29, 2],
            vec![89, 15, 72, 70, 39],
            vec![99, 79, 87, 93, 34],
            vec![53, 71, 10, 20, 50],
        ],
        vec![
            vec![76, 55, 18, 28, 33],
            vec![37, 14, 64, 7, 0],
            vec![13, 60, 54, 62, 5],
            vec![61, 3, 99, 56, 10],
            vec![30, 86, 47, 24, 39],
        ],
        vec![
            vec![24, 96, 18, 55, 52],
            vec![79, 73, 91, 14, 88],
            vec![42, 37, 12, 64, 21],
            vec![45, 51, 33, 34, 3],
            vec![8, 77, 47, 40, 16],
        ],
        vec![
            vec![55, 54, 98, 85, 97],
            vec![80, 21, 27, 15, 36],
            vec![1, 26, 7, 70, 44],
            vec![48, 25, 59, 0, 38],
            vec![5, 58, 18, 3, 12],
        ],
        vec![
            vec![76, 78, 79, 92, 88],
            vec![7, 80, 57, 68, 28],
            vec![27, 16, 8, 10, 62],
            vec![32, 98, 64, 60, 39],
            vec![52, 4, 85, 48, 35],
        ],
        vec![
            vec![94, 64, 19, 67, 12],
            vec![49, 61, 77, 89, 31],
            vec![80, 11, 18, 26, 83],
            vec![21, 76, 66, 85, 91],
            vec![60, 88, 28, 86, 69],
        ],
        vec![
            vec![88, 27, 52, 28, 89],
            vec![3, 5, 15, 78, 97],
            vec![79, 34, 7, 42, 80],
            vec![14, 83, 90, 68, 65],
            vec![35, 13, 58, 71, 17],
        ],
        vec![
            vec![90, 10, 0, 30, 69],
            vec![64, 75, 61, 62, 97],
            vec![3, 29, 40, 58, 57],
            vec![98, 48, 5, 37, 23],
            vec![70, 12, 6, 36, 7],
        ],
        vec![
            vec![27, 87, 59, 84, 83],
            vec![19, 91, 24, 32, 78],
            vec![62, 14, 58, 18, 68],
            vec![37, 1, 99, 44, 94],
            vec![71, 23, 54, 8, 74],
        ],
        vec![
            vec![12, 78, 7, 55, 59],
            vec![81, 87, 69, 49, 22],
            vec![79, 99, 84, 58, 65],
            vec![40, 42, 85, 74, 91],
            vec![61, 39, 51, 88, 36],
        ],
        vec![
            vec![2, 9, 62, 76, 11],
            vec![87, 36, 72, 80, 96],
            vec![28, 16, 7, 17, 39],
            vec![68, 33, 5, 71, 92],
            vec![41, 22, 70, 4, 52],
        ],
        vec![
            vec![49, 1, 23, 37, 60],
            vec![72, 79, 2, 40, 13],
            vec![31, 47, 59, 48, 33],
            vec![96, 80, 62, 3, 12],
            vec![34, 89, 75, 18, 95],
        ],
        vec![
            vec![34, 72, 3, 78, 60],
            vec![57, 51, 47, 26, 7],
            vec![2, 50, 82, 4, 56],
            vec![81, 95, 22, 42, 73],
            vec![25, 37, 97, 65, 6],
        ],
        vec![
            vec![86, 23, 17, 39, 53],
            vec![6, 49, 66, 32, 30],
            vec![71, 40, 64, 11, 8],
            vec![82, 60, 18, 13, 68],
            vec![12, 7, 42, 52, 72],
        ],
        vec![
            vec![16, 0, 88, 84, 98],
            vec![42, 92, 19, 89, 2],
            vec![51, 26, 1, 33, 10],
            vec![40, 87, 32, 17, 27],
            vec![7, 68, 35, 50, 11],
        ],
        vec![
            vec![39, 52, 56, 23, 75],
            vec![66, 64, 26, 10, 53],
            vec![46, 25, 60, 48, 28],
            vec![61, 27, 98, 40, 59],
            vec![65, 95, 85, 13, 62],
        ],
        vec![
            vec![74, 32, 72, 43, 23],
            vec![26, 90, 83, 68, 82],
            vec![48, 60, 39, 17, 54],
            vec![51, 36, 37, 85, 88],
            vec![99, 81, 69, 50, 93],
        ],
        vec![
            vec![16, 71, 3, 29, 81],
            vec![18, 97, 17, 20, 48],
            vec![95, 38, 61, 87, 98],
            vec![52, 76, 8, 42, 36],
            vec![45, 4, 78, 55, 89],
        ],
        vec![
            vec![64, 18, 54, 39, 77],
            vec![36, 66, 98, 88, 38],
            vec![49, 50, 74, 69, 65],
            vec![71, 3, 93, 34, 82],
            vec![23, 26, 92, 15, 33],
        ],
        vec![
            vec![18, 99, 45, 69, 8],
            vec![32, 75, 47, 2, 91],
            vec![41, 55, 90, 5, 3],
            vec![93, 11, 84, 78, 56],
            vec![80, 57, 51, 50, 74],
        ],
        vec![
            vec![90, 0, 19, 53, 11],
            vec![69, 50, 47, 16, 26],
            vec![5, 32, 73, 51, 14],
            vec![84, 37, 34, 7, 56],
            vec![68, 10, 74, 29, 62],
        ],
        vec![
            vec![42, 80, 7, 72, 35],
            vec![1, 50, 15, 0, 49],
            vec![43, 19, 41, 26, 48],
            vec![65, 2, 36, 17, 20],
            vec![85, 32, 5, 75, 30],
        ],
        vec![
            vec![8, 80, 72, 1, 81],
            vec![90, 69, 48, 36, 23],
            vec![55, 16, 35, 41, 5],
            vec![71, 12, 59, 29, 79],
            vec![60, 92, 53, 73, 96],
        ],
        vec![
            vec![45, 55, 75, 77, 6],
            vec![1, 91, 76, 5, 39],
            vec![54, 65, 61, 34, 8],
            vec![27, 59, 47, 85, 44],
            vec![66, 29, 36, 80, 60],
        ],
        vec![
            vec![82, 75, 32, 29, 14],
            vec![41, 21, 20, 68, 89],
            vec![80, 64, 56, 33, 31],
            vec![13, 12, 55, 81, 7],
            vec![36, 78, 4, 24, 50],
        ],
        vec![
            vec![55, 28, 90, 35, 22],
            vec![77, 92, 56, 16, 47],
            vec![48, 4, 67, 95, 37],
            vec![63, 80, 36, 12, 44],
            vec![88, 58, 10, 68, 84],
        ],
        vec![
            vec![7, 88, 53, 55, 60],
            vec![81, 99, 91, 28, 70],
            vec![75, 8, 0, 40, 84],
            vec![17, 24, 30, 71, 56],
            vec![78, 20, 83, 29, 74],
        ],
        vec![
            vec![10, 16, 11, 5, 38],
            vec![72, 77, 37, 68, 81],
            vec![78, 13, 45, 89, 46],
            vec![90, 49, 18, 53, 42],
            vec![71, 0, 28, 26, 75],
        ],
        vec![
            vec![36, 73, 23, 44, 7],
            vec![1, 96, 17, 11, 42],
            vec![77, 54, 68, 80, 79],
            vec![85, 56, 28, 58, 52],
            vec![57, 2, 43, 18, 61],
        ],
        vec![
            vec![9, 18, 29, 44, 61],
            vec![13, 80, 57, 95, 47],
            vec![2, 15, 72, 50, 65],
            vec![77, 12, 42, 64, 36],
            vec![49, 67, 31, 6, 90],
        ],
        vec![
            vec![99, 74, 9, 64, 59],
            vec![17, 87, 86, 5, 28],
            vec![44, 41, 82, 53, 45],
            vec![95, 51, 98, 50, 94],
            vec![22, 37, 46, 25, 4],
        ],
        vec![
            vec![46, 54, 82, 17, 77],
            vec![88, 63, 50, 65, 64],
            vec![5, 19, 39, 21, 11],
            vec![57, 58, 20, 6, 40],
            vec![99, 37, 66, 70, 95],
        ],
        vec![
            vec![30, 68, 11, 29, 13],
            vec![1, 69, 0, 9, 93],
            vec![33, 57, 23, 50, 71],
            vec![8, 77, 6, 25, 45],
            vec![34, 12, 60, 7, 28],
        ],
        vec![
            vec![91, 53, 88, 47, 40],
            vec![77, 92, 87, 37, 84],
            vec![89, 95, 78, 21, 39],
            vec![65, 20, 54, 94, 85],
            vec![45, 74, 30, 49, 11],
        ],
        vec![
            vec![3, 58, 90, 93, 96],
            vec![53, 66, 24, 43, 32],
            vec![62, 84, 19, 82, 22],
            vec![13, 89, 20, 97, 1],
            vec![15, 91, 51, 68, 49],
        ],
    ]
}
