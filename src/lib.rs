use rand::prelude::*;
// SGD - Stochastic Gradient Descent
// Initialize parameters (model weights) to set path for algorithm
// TODO: Randomly select data points from dataset
// TODO: Calculate gradient for loss function of the selected data points
pub fn stoch_gradient_descent(dataset: Vec<f32>) -> Vec<f32> {
    let mut rng = thread_rng();
    let weights: Vec<f32> = vec![rng.gen(); dataset.len()];
    weights
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let dataset = vec![0.1, -0.2, 0.2, 0.3, 0.9];
        let result = stoch_gradient_descent(dataset);
        assert_eq!(result.len(), 5);
    }
}
