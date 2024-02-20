
use crate::structure::Complex;

///拉普拉斯变换
/// # Example
/// ```
/// use algori::structure::Complex;
/// use algori::math::laplace_transform;
/// let s = vec![vec![1.0,2.0,1.0],vec![2.0,4.0,2.0],vec![1.0,2.0,1.0]];
/// let r = laplace_transform(&s);
/// assert_eq!(r,[[0.0,0.0,0.0],[0.0,-8.0,0.0],[0.0,0.0,0.0]]);
/// ```
pub fn laplace_transform(input: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut laplace_output = vec![vec![0.0; input[0].len()]; input.len()];
    for i in 1..input.len()-1 {
        for j in 1..input[i].len()-1 {
            let laplace_value = input[i-1][j] + input[i+1][j] + input[i][j-1] + input[i][j+1] - 4.0 * input[i][j];
            laplace_output[i][j] = laplace_value;
        }
    }
    laplace_output
}

