use tours::solver;

mod tours;

fn main() {
    let success = solver::Solver::new().solve();
    println!("Success: {}!", success);
}
