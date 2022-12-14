use anyhow::Result;
use itertools::Itertools;
use std::{
    io::{self, BufRead},
    ops::Range,
};

/*
    Two options:

    1. For every tree not in the border, check if there's any tree with the same or bigger height in
    any direction. If that's true, then the tree is visible. Otherwise, it isn't.

    Complexity:
        r = num trees in a row
        c = num trees in a column

        O((r * c) * (r + c)) = O(r²c + rc²)

    2. For each row of trees, traverse it leftwards and rightwards, and mark visible trees. And for each
    column of trees, traverse it upwards and downwards, and mark visible trees.

    Complexity:
        r = num trees in a row
        c = num trees in a column

        O(2rc + 2cr) = O(rc)


    >>> We're using the 2nd option
*/
pub fn solve() -> Result<()> {
    let forest = parse_forest()?;
    let visible_trees = count_visible_trees(&forest);
    println!("{}", visible_trees);
    Ok(())
}

struct Forest {
    trees: Vec<Vec<u32>>,
}

impl Forest {
    fn new() -> Self {
        Self { trees: vec![] }
    }
}

fn parse_forest() -> Result<Forest> {
    let mut forest = Forest::new();
    for line in io::stdin().lock().lines() {
        let line = line?.trim().to_string();
        forest.trees.push(Vec::from_iter(
            line.chars()
                .into_iter()
                .map(|c| c.to_digit(10).ok_or("Could not parse digit").unwrap()),
        ));
    }
    Ok(forest)
}

fn count_visible_trees(forest: &Forest) -> usize {
    let mut tree_visibility = forest
        .trees
        .iter()
        .map(|v| v.iter().map(|_| false).collect_vec())
        .collect_vec();

    let rows = forest.trees.len();
    let cols = forest.trees[0].len();

    make_trees_in_borders_visible(&mut tree_visibility, cols, rows);
    update_trees_visibility(forest, &mut tree_visibility, cols, rows);

    tree_visibility
        .into_iter()
        .map(|v| v.into_iter().filter(|&b| b).count())
        .sum()
}

fn make_trees_in_borders_visible(visibility: &mut [Vec<bool>], cols: usize, rows: usize) {
    let col_range = RangeGenerator::new(0..cols);
    let row_range = RangeGenerator::new(0..rows);

    let trees_north_row = col_range.gen().map(|c| (0, c)).collect_vec();
    let trees_south_row = col_range
        .gen()
        .map(|c| (row_range.end() - 1, c))
        .collect_vec();
    let trees_west_col = row_range.gen().map(|r| (r, 0)).collect_vec();
    let trees_east_col = row_range
        .gen()
        .map(|r| (r, col_range.end() - 1))
        .collect_vec();

    let mut make_tree_visible = |coords: &[(usize, usize)]| {
        for (row, col) in coords {
            let (row, col) = (*row, *col);
            visibility[row][col] = true;
        }
    };
    make_tree_visible(&trees_north_row);
    make_tree_visible(&trees_south_row);
    make_tree_visible(&trees_west_col);
    make_tree_visible(&trees_east_col);
}

struct RangeGenerator {
    range: Range<usize>,
}

impl RangeGenerator {
    fn new(range: Range<usize>) -> Self {
        Self { range }
    }

    fn gen(&self) -> Range<usize> {
        self.range.clone()
    }

    fn end(&self) -> usize {
        self.range.end
    }
}

fn update_trees_visibility(
    forest: &Forest,
    visibility: &mut [Vec<bool>],
    cols: usize,
    rows: usize,
) {
    let mut update_tree_visibility = |mut min_height: u32, (row, col): (usize, usize)| {
        let tree_height = forest.trees[row][col];
        if min_height < tree_height {
            min_height = tree_height;
            visibility[row][col] = true;
        }
        min_height
    };

    for row in 1..(rows - 1) {
        let mut min_height = forest.trees[row][0];
        for col in 0..(cols - 1) {
            min_height = update_tree_visibility(min_height, (row, col));
        }

        let mut min_height = forest.trees[row][cols - 1];
        for col in (1..cols).rev() {
            min_height = update_tree_visibility(min_height, (row, col));
        }
    }

    for col in 1..(cols - 1) {
        let mut min_height = forest.trees[0][col];
        for row in 0..(rows - 1) {
            min_height = update_tree_visibility(min_height, (row, col));
        }

        let mut min_height = forest.trees[rows - 1][col];
        for row in (1..rows).rev() {
            min_height = update_tree_visibility(min_height, (row, col));
        }
    }
}
