use std::ops::{Index, IndexMut, Mul};

use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2x2 {
    data: [[f64; 2]; 2],
}
impl Matrix2x2 {
    pub const _IDENTITY: Self = Matrix2x2 {
        data: [[1., 0.], [0., 1.]],
    };
    pub fn new(data: [[f64; 2]; 2]) -> Self {
        Self { data }
    }
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
    pub fn inverse(&self) -> Self {
        let determinant_inv = 1.0 / self.determinant();
        Self::new([[self[1][1] * determinant_inv, -self[1][0] * determinant_inv], [-self[0][1] * determinant_inv, self[0][0] * determinant_inv]])
    }
}
impl Index<usize> for Matrix2x2 {
    type Output = [f64; 2];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl IndexMut<usize> for Matrix2x2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl Mul for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [[0.0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    data[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        Matrix2x2::new(data)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3x3 {
    data: [[f64; 3]; 3],
}
impl Matrix3x3 {
    pub const _IDENTITY: Matrix3x3 = Matrix3x3 {
        data: ([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]),
    };
    pub fn new(data: [[f64; 3]; 3]) -> Self {
        Self { data }
    }
    pub fn transpose(&self) -> Self {
        let mut data = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                data[j][i] = self[i][j];
            }
        }

        Matrix3x3::new(data)
    }
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2x2 {
        if row >= 3 || col >= 3 {
            panic!("Index out of bounds! row {}, col {}", row, col)
        }
        let mut data = [[0.0; 2]; 2];
        let mut new_i = 0;
        for i in 0..3 {
            if i != row {
                let mut new_j = 0;
                for j in 0..3 {
                    if j != col {
                        data[new_i][new_j] = self[i][j];
                        new_j += 1;
                    }
                }
                new_i += 1;
            }
        }
        Matrix2x2::new(data)
    }
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) & 1 == 1 {
            return -self.minor(row, col);
        }
        return self.minor(row, col);
    }
    pub fn determinant(&self) -> f64 {
        let mut det = 0.;
        for i in 0..3 {
            det += self[0][i] * self.cofactor(0, i);
        }
        det
    }
    pub fn inverse(&self) -> Self {
        let d = self.determinant();
        let mut data = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                let c = self.cofactor(i, j);
                data[j][i] = c / d;
            }
        }
        Matrix3x3::new(data)
    }
}
impl Index<usize> for Matrix3x3 {
    type Output = [f64; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl IndexMut<usize> for Matrix3x3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl Mul for Matrix3x3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    data[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        Matrix3x3::new(data)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Matrix4x4 {
    data: [[f64; 4]; 4],
}
impl Matrix4x4 {
    pub const _IDENTITY: Self = Matrix4x4 {
        data: [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ],
    };
    pub fn new(data: [[f64; 4]; 4]) -> Self {
        Self { data }
    }
    pub fn transpose(&self) -> Self {
        let mut data = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                data[j][i] = self[i][j];
            }
        }

        Matrix4x4::new(data)
    }
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3x3 {
        if row >= 4 || col >= 4 {
            panic!("Index out of bounds! row {}, col {}", row, col)
        }
        let mut data = [[0.0; 3]; 3];
        let mut new_i = 0;
        for i in 0..4 {
            if i != row {
                let mut new_j = 0;
                for j in 0..4 {
                    if j != col {
                        data[new_i][new_j] = self[i][j];
                        new_j += 1;
                    }
                }
                new_i += 1;
            }
        }
        Matrix3x3::new(data)
    }
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) & 1 == 1 {
            return -self.minor(row, col);
        }
        return self.minor(row, col);
    }
    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for i in 0..4 {
            det += self[0][i] * self.cofactor(0, i);
        }
        det
    }
    pub fn inverse(&self) -> Self {
        let d = self.determinant();
        let mut data = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let c = self.cofactor(i, j);
                data[j][i] = c / d;
            }
        }
        Matrix4x4::new(data)
    }
}
impl Index<usize> for Matrix4x4 {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    data[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        Matrix4x4::new(data)
    }
}
impl Mul<Tuple> for Matrix4x4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple::tuple(
            self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3] * rhs.w,
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3] * rhs.w,
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3] * rhs.w,
            self[3][0] * rhs.x + self[3][1] * rhs.y + self[3][2] * rhs.z + self[3][3] * rhs.w,
        )
    }
}
impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if (self[i][j] - other[i][j]).abs() > 1e-5 {
                    println!("{}-{}={}", self[i][j], other[i][j], self[i][j] - other[i][j]);
                    return false;
                }
            }
        }
        return true;
    }
}