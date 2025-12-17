# -Path-Finder-1-can-you-reach-the-exit---4-kyu
You are at position [0, 0] in maze NxN and you can only move in one of the four cardinal directions (i.e. North, East, South, West). Return true if you can reach position [N-1, N-1] or false otherwise.

Empty positions are marked ..
Walls are marked W.
Start and exit positions are empty in all test cases.


#[cfg(test)]
mod tests {
    use super::path_finder;

    #[test]
    fn basic() {
        test_maze("\
            .W.\n\
            .W.\n\
            ...\
            ",
            true,
        );
        
        test_maze("\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            ......\
            ",
            true,
        );
        
        test_maze("\
            ......\n\
            ......\n\
            ......\n\
            ......\n\
            .....W\n\
            ....W.\
            ",
            false,
        );
    }
    
    fn test_maze(maze: &str, expect: bool) {
        let actual = path_finder(maze);
        
        assert!(
            actual == expect,
            "Test failed!\n\
             Got:      {}\n\
             Expected: {}\n\
             Maze was: \n\
             {}",
            actual,
            expect, 
            maze
        );
    }
}
