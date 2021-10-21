pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] == '0' {
            return;
        }
        grid[i][j] = '0';
        if i > 0 {
            dfs(grid, i - 1, j);
        }
        if i < grid.len() - 2 {
            dfs(grid, i + 1, j);
        }
        if j > 0 {
            dfs(grid, i, j - 1);
        }
        if j < grid[0].len() - 2 {
            dfs(grid, i, j + 1);
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' {
                count += 1;
                dfs(&mut grid, i, j);
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
    }
}
