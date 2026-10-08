use tours::reentrant_tour_solver;

use crate::tours::{Point, red_faced_cube_solver};

mod tours;

fn main() {
    let mut red_faced_cube = red_faced_cube_solver::RedFacedCubeSolver::new();
    let red_faced_cube_result = red_faced_cube.solve();

    let start_end = Point { row: 1, col: 3 };
    let mut reentrant_tour = reentrant_tour_solver::ReentrantTourSolver::new(start_end);
    let reentrant_tour_result = reentrant_tour.solve();

    match red_faced_cube_result {
        Some(solution) => {
            println!("Red-Faced Cube solution found!");
            tours::print_board(Point { row: 0, col: 0 }, &solution.rolls, false);
        }
        None => println!("Red-Faced Cube solution found!"),
    }

    match reentrant_tour_result {
        Some(solution) => {
            println!("Reentrant Tour solution found!");
            tours::print_board(start_end, &solution.rolls, false);
        }
        None => println!("No Reentrant Tour solution found!"),
    }
}
