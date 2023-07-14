//! This crate allows easy access to instances of the traveling salesman problem taken from tsplib.

use anyhow::{anyhow, Result};
use include_dir::{include_dir, Dir};
use mahf::{
    problems::{ObjectiveFunction, Problem, VectorProblem},
    SingleObjective,
};
use tspf::WeightKind;

mod instances;
mod opt;

pub use instances::Instances;

type Edge = (usize, usize);
pub type Node = usize;
pub type Route = Vec<Node>;

static FILES: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/tsplib");

/// Represents the (global) optimum of the search space.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TspOptimum {
    pub objective: SingleObjective,
    pub solution: Option<Route>,
}

/// Represents an instance of the symmetric travelling salesman problem.
#[derive(serde::Serialize)]
pub struct Tsp {
    best_solution: Option<TspOptimum>,
    #[serde(skip)]
    inner: tspf::Tsp,
}

impl Tsp {
    fn evaluate_solution(&self, solution: &Route) -> SingleObjective {
        solution
            .iter()
            .copied()
            .zip(solution.iter().copied().skip(1))
            .map(|edge| self.distance(edge))
            .sum::<f64>()
            .try_into()
            .unwrap()
    }
}

impl From<Instances> for Tsp {
    fn from(value: Instances) -> Self {
        value.load()
    }
}

impl Tsp {
    /// Returns the instance name.
    pub fn name(&self) -> &str {
        self.inner.name()
    }

    /// Returns the best known solution, if it exists.
    pub fn best_solution(&self) -> Option<&TspOptimum> {
        self.best_solution.as_ref()
    }

    /// Returns the best known fitness value, if it exists.
    pub fn best_fitness(&self) -> Option<f64> {
        self.best_solution.as_ref().map(|o| o.objective.value())
    }

    /// Returns the weight/distance of the given edge.
    pub fn distance(&self, edge: Edge) -> f64 {
        let (a, b) = edge;

        // TODO: this seems like a bug in tspf
        if self.inner.weight_kind() == WeightKind::Explicit {
            self.inner.weight(a, b)
        } else {
            self.inner.weight(a + 1, b + 1)
        }
    }

    /// Greedily constructs a Route, always taking the shortest edge.
    pub fn greedy_route(&self) -> Route {
        let mut route = Vec::with_capacity(self.inner.dim());
        let mut remaining = (1..self.inner.dim()).collect::<Vec<usize>>();
        route.push(0);
        while !remaining.is_empty() {
            let last = *route.last().unwrap();
            let next_index = remaining
                .iter()
                .enumerate()
                .min_by_key(|(_, &r)| SingleObjective::try_from(self.distance((last, r))).unwrap())
                .unwrap()
                .0;
            let next = remaining.remove(next_index);
            route.push(next);
        }
        route
    }

    /// This method constructs a TSP instance from a string representation (`data`) and an optional solution (`opt`).
    /// There is no good reason to call it directly, just use [Instances::try_load()] instead.
    pub fn try_parse(data: &str, opt: Option<&str>) -> Result<Self> {
        let tsp =
            tspf::TspBuilder::parse_str(data).map_err(|err| anyhow!("parsing failed: {}", err))?;

        let mut instance = Tsp {
            best_solution: None,
            inner: tsp,
        };

        if let Some(opt) = opt {
            instance.best_solution = Some(opt::parse_opt_file(&instance, opt));
        }

        Ok(instance)
    }
}

impl Problem for Tsp {
    type Encoding = Route;
    type Objective = SingleObjective;

    fn name(&self) -> &str {
        "Tsp"
    }
}

impl ObjectiveFunction for Tsp {
    fn objective(&self, solution: &Self::Encoding) -> Self::Objective {
        self.evaluate_solution(solution)
    }
}

impl VectorProblem for Tsp {
    type Element = usize;

    fn dimension(&self) -> usize {
        self.inner.dim()
    }
}
