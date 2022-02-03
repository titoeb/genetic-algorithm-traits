/// A single instance in the genetic algorithm.
/// In a TSP for example, this would be and individual route.
pub trait Individual<'a>: Clone {
    /// The Type of cost data this individual is compatible to compute its
    /// fitness on.
    type IndividualCost: 'a;
    /// Randomly change the object and therefore it's fitness
    /// This is a key step of the genetic algorithm.
    ///
    /// # Arguments
    ///
    /// * `prob` - The probability with which the individual will mutate.
    fn mutate(self, prob: f32) -> Self;
    /// The `crossover` takes two individual and combines their characteristics.
    /// The implementation depends on the problem to be solve with genetic algorithm but
    /// both from a performance and runtime perspective, this is one of the most important
    /// and time-consuming methods.
    ///
    /// # Arguments
    ///
    /// * `other` - The other individual that should be `crossover`ed with.
    ///
    fn crossover(&self, other: &Self) -> Self;
    /// How `fit` is your individual, e.g. how well does it solve the problem you are
    /// trying to solve with genetic algorithms. This is the metric that is maximised, e.g.
    /// overall individuals with a very high fitness should be found.
    ///
    /// # Arguments
    ///
    /// * `cost_data` - The data that might be needed to compute your fitness. If you use
    /// genetic algorithm to solve a traveling salesman problem, the `cost_data` will typically
    /// contain your distance matrix.
    ///
    fn fitness(&self, cost_data: &Self::IndividualCost) -> f64;
}
