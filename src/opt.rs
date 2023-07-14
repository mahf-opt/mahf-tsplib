use mahf::SingleObjective;

use crate::{Route, Tsp, TspOptimum};

pub fn parse_opt_file(instance: &Tsp, opt_contents: &str) -> TspOptimum {
    let best_solution = opt_contents
        .lines()
        .find(|line| line.starts_with("BEST_SOLUTION"))
        .map(|line| &line["BEST_SOLUTION: ".len()..])
        .map(str::parse::<f64>)
        .map(Result::unwrap)
        .map(SingleObjective::try_from)
        .map(Result::unwrap);

    let route = opt_contents
        .lines()
        .skip_while(|&line| line != "TOUR_SECTION")
        .skip(1)
        .take_while(|&line| line != "-1")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .map(|x| x - 1)
        .collect::<Route>();

    let solution = if route.is_empty() { None } else { Some(route) };

    assert!(best_solution.is_some() || solution.is_some());

    let objective =
        best_solution.unwrap_or_else(|| instance.evaluate_solution(solution.as_ref().unwrap()));

    TspOptimum {
        objective,
        solution,
    }
}
