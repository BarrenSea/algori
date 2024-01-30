use std::ops::{Add, Sub, Mul, Div};
use std::cmp::{PartialEq, Eq};

#[derive(Debug, PartialEq,Clone)]
///矩阵
///rows代表矩阵的行
///cols代表矩阵的列
///# Examples
///```
///    use algori::matrix::Matrix;
///    let matrix1 = Matrix::new(5, 10, vec![
///        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
///        vec![11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0],
///        vec![21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0],
///        vec![31.0, 32.0, 33.0, 34.0, 35.0, 36.0, 37.0, 38.0, 39.0, 40.0],
///        vec![41.0, 42.0, 43.0, 44.0, 45.0, 46.0, 47.0, 48.0, 49.0, 50.0],
///    ]);
///
///    let matrix2 = Matrix::new(10, 3, vec![
///        vec![1.0, 2.0, 3.0],
///        vec![4.0, 5.0, 6.0],
///        vec![7.0, 8.0, 9.0],
///        vec![10.0, 11.0, 12.0],
///        vec![13.0, 14.0, 15.0],
///        vec![16.0, 17.0, 18.0],
///        vec![19.0, 20.0, 21.0],
///        vec![22.0, 23.0, 24.0],
///        vec![25.0, 26.0, 27.0],
///       vec![28.0, 29.0, 30.0],
///    ]);
///
///    let product_matrix = matrix1 * matrix2;
///    
///```
///
///
///
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<Vec<f64>>) -> Self {
        Matrix { rows, cols, data }
    }
    ///检查矩阵是否正确,正确返回Some(1)
    pub fn check(&self) -> Option<usize> {
	if self.data.len() != self.rows {
            return None; // 行数不匹配
        }
        if self.data.iter().any(|row| row.len() != self.cols) {
            return None; // 列数不匹配
        }
	Some(1)
    }
}

impl Add for Matrix {
    type Output = Matrix;
    ///矩阵加法
    fn add(self, other: Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        let data: Vec<Vec<f64>> = self.data
            .iter()
            .zip(other.data.iter())
            .map(|(row1, row2)| {
                row1.iter()
                    .zip(row2.iter())
                    .map(|(&a, &b)| a + b)
                    .collect()
            })
            .collect();

        Matrix::new(self.rows, self.cols, data)
    }
}

impl Sub for Matrix {
    type Output = Matrix;
    ///矩阵减法
    fn sub(self, other: Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        let data: Vec<Vec<f64>> = self.data
            .iter()
            .zip(other.data.iter())
            .map(|(row1, row2)| {
                row1.iter()
                    .zip(row2.iter())
                    .map(|(&a, &b)| a - b)
                    .collect()
            })
            .collect();

        Matrix::new(self.rows, self.cols, data)
    }
}


impl Mul for Matrix {
    type Output = Matrix;
    ///矩阵乘法
    fn mul(self, other: Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);

        let mut result = vec![vec![0.0; other.cols]; self.rows];
        
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }

        Matrix::new(self.rows, other.cols, result)
    }
}




