mod maze;

use maze::*;

fn main() {
    let mut maze = Maze::new("mazes/maze1.txt");
    maze.solve();
    maze.print();
}
