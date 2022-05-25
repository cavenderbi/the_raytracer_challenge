#[cfg(test)]
mod tests {
    use crate::{canvas::Canvas, color::Color, matrix::*, tuple::Tuple};
    // Linear Algebra tests.
    #[test]
    fn point_w() {
        assert_eq!(
            Tuple::point(4.3, -4.2, 3.1),
            Tuple::tuple(4.3, -4.2, 3.1, 1.0),
        )
    }
    #[test]
    fn vector_w() {
        assert_eq!(
            Tuple::vector(4.3, -4.2, 3.1),
            Tuple::tuple(4.3, -4.2, 3.1, 0.0),
        )
    }
    #[test]
    fn add_tuples() {
        assert_eq!(
            Tuple::tuple(3.0, -2.0, 5.0, 1.0) + Tuple::tuple(-2.0, 3.0, 1.0, 0.0),
            Tuple::tuple(1.0, 1.0, 6.0, 1.0),
        )
    }
    #[test]
    fn sub_points() {
        assert_eq!(
            Tuple::point(3.0, 2.0, 1.0) - Tuple::point(5.0, 6.0, 7.0),
            Tuple::vector(-2.0, -4.0, -6.0),
        );
        assert_eq!(
            Tuple::point(3.0, 2.0, 1.0) - Tuple::vector(5.0, 6.0, 7.0),
            Tuple::point(-2.0, -4.0, -6.0)
        );
        assert_eq!(
            Tuple::vector(3.0, 2.0, 1.0) - Tuple::vector(5.0, 6.0, 7.0),
            Tuple::vector(-2.0, -4.0, -6.0),
        );
    }
    #[test]
    fn mul_tuple() {
        assert_eq!(
            Tuple::tuple(1.0, -2.0, 3.0, -4.0) * 3.5,
            Tuple::tuple(3.5, -7.0, 10.5, -14.0),
        );
        assert_eq!(
            Tuple::tuple(1.0, -2.0, 3.0, -4.0) * 0.5,
            Tuple::tuple(0.5, -1.0, 1.5, -2.0),
        );
    }
    #[test]
    fn div_tuple() {
        assert_eq!(
            Tuple::tuple(1.0, -2.0, 3.0, -4.0) / 2.0,
            Tuple::tuple(0.5, -1.0, 1.5, -2.0),
        )
    }
    #[test]
    fn vec_length() {
        assert_eq!(Tuple::vector(1.0, 0.0, 0.0).length(), 1.0);
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0).length(), 1.0);
        assert_eq!(Tuple::vector(0.0, 0.0, 1.0).length(), 1.0);
        assert_eq!(Tuple::vector(1.0, 2.0, 3.0).length(), 14.0_f64.sqrt());
        assert_eq!(Tuple::vector(-1.0, -2.0, -3.0).length(), 14.0_f64.sqrt());
    }
    #[test]
    fn vec_norm() {
        assert_eq!(
            Tuple::vector(4.0, 0.0, 0.0).normalize(),
            Tuple::vector(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Tuple::vector(1.0, 2.0, 3.0).normalize(),
            Tuple::vector(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        );

        let norm = Tuple::vector(1.0, 2.0, 3.0).normalize();
        assert_eq!(norm.length(), 1.0);
    }
    #[test]
    fn vec_dot() {
        assert_eq!(
            Tuple::dot(Tuple::vector(1.0, 2.0, 3.0), Tuple::vector(2.0, 3.0, 4.0)),
            20.0
        );
    }
    #[test]
    fn vec_cross() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(Tuple::cross(a, b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(Tuple::cross(b, a), Tuple::vector(1.0, -2.0, 1.0));
    }
    // Matrix tests.
    #[test]
    fn matrix_2x2() {
        let m = Matrix2x2::new([[-3., 5.], [1., -2.]]);
        assert_eq!(m[0][0], -3.);
        assert_eq!(m[0][1], 5.);
        assert_eq!(m[1][0], 1.);
        assert_eq!(m[1][1], -2.);
    }
    #[test]
    fn matrix_3x3() {
        let m = Matrix3x3::new([[-3., 5., 0.], [1., -2., -7.], [0., 1., 1.]]);
        assert_eq!(m[0][0], -3.);
        assert_eq!(m[1][1], -2.);
        assert_eq!(m[2][2], 1.);
    }
    #[test]
    fn matrix_4x4() {
        let m = Matrix4x4::new([
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m[0][0], 1.);
        assert_eq!(m[0][3], 4.);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }
    #[test]
    fn mul_matrix_4x4() {
        let a = Matrix4x4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.],
        ]);
        let b = Matrix4x4::new([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.],
        ]);
        let ab = Matrix4x4::new([
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.],
        ]);
        assert_eq!(a * b, ab);
    }
    #[test]
    fn mul_matrix_tuple() {
        let a = Matrix4x4::new([
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.],
        ]);
        let b = Tuple::tuple(1., 2., 3., 1.);
        let ab = Tuple::tuple(18., 24., 33., 1.);
        assert_eq!(a * b, ab);
    }
    #[test]
    fn identity_matrix() {
        let a = Matrix4x4::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        assert_eq!(Matrix4x4::_IDENTITY, a);
    }
    #[test]
    fn transpose_matrix() {
        let a = Matrix4x4::new([
            [0., 9., 3., 0.],
            [9., 8., 0., 8.],
            [1., 8., 5., 3.],
            [0., 0., 5., 8.],
        ]);
        let aT = Matrix4x4::new([
            [0., 9., 1., 0.],
            [9., 8., 8., 0.],
            [3., 0., 5., 5.],
            [0., 8., 3., 8.],
        ]);
        assert_eq!(a.transpose(), aT);
    }
    #[test]
    fn transpose_identity_matrix() {
        assert_eq!(Matrix4x4::_IDENTITY, Matrix4x4::_IDENTITY.transpose());
    }
    #[test]
    fn matrix_2x2_determinant() {
        let a = Matrix2x2::new([[1., 5.], [-3., 2.]]);
        assert_eq!(a.determinant(), 17.);
    }
    #[test]
    fn matrix_3x3_determinant() {
        let a = Matrix3x3::new([[1., 2., 6.], [-5., 8., -4.], [2., 6., 4.]]);
        assert_eq!(a.cofactor(0, 0), 56.);
        assert_eq!(a.cofactor(0, 1), 12.);
        assert_eq!(a.cofactor(0, 2), -46.);
        assert_eq!(a.determinant(), -196.);
    }
    #[test]
    fn matrix_3x3_submatrix() {
        let a = Matrix3x3::new([[1., 5., 0.], [-3., 2., 7.], [0., 6., -3.]]);
        let sub = Matrix2x2::new([[-3., 2.], [0., 6.]]);
        assert_eq!(a.submatrix(0, 2), sub);
    }
    #[test]
    fn matrix_4x4_submatrix() {
        let a = Matrix4x4::new([
            [-6., 1., 1., 6.],
            [-8., 5., 8., 6.],
            [-1., 0., 8., 2.],
            [-7., 1., -1., 1.],
        ]);
        let sub = Matrix3x3::new([[-6., 1., 6.], [-8., 8., 6.], [-7., -1., 1.]]);
        assert_eq!(a.submatrix(2, 1), sub);
    }
    #[test]
    fn matrix_3x3_minor() {
        let a = Matrix3x3::new([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);
        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), a.minor(1, 0))
    }
    #[test]
    fn matrix_3x3_cofactor() {
        let a = Matrix3x3::new([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);
        assert_eq!(a.minor(0, 0), -12.);
        assert_eq!(a.cofactor(0, 0), -12.);
        assert_eq!(a.minor(1, 0), 25.);
        assert_eq!(a.cofactor(1, 0), -25.)
    }
    #[test]
    fn matrix_4x4_inverse() {
        let a = Matrix4x4::new([
            [-5., 2., 6., -8.],
            [1., -5., 1., 8.],
            [7., 7., -6., -7.],
            [1., -3., 7., 4.],
        ]);
        let i = a.inverse();
        assert_eq!(a.determinant(), 532.);
        assert_eq!(a.cofactor(2, 3), -160.);
        assert_eq!(i[3][2], -160. / 532.);
        assert_eq!(a.cofactor(3, 2), 105.);
        assert_eq!(i[2][3], 105. / 532.);
        assert_eq!(
            i,
            Matrix4x4::new([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ])
        );

        let a = Matrix4x4::new([
            [8., -5., 9., 2.],
            [7., 5., 6., 1.],
            [-6., 0., 9., 6.],
            [-3., 0., -9., -4.],
        ]);
        let i = Matrix4x4::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert_eq!(a.inverse(), i);

        let a = Matrix4x4::new([
            [9., 3., 0., 9.],
            [-5., -2., -6., -3.],
            [-4., 9., 6., 4.],
            [-7., 6., 6., 2.],
        ]);
        let i = Matrix4x4::new([
            [-0.04074, -0.07778,  0.14444, -0.22222], 
            [-0.07778,  0.03333,  0.36667, -0.33333], 
            [-0.02901, -0.14630, -0.10926,  0.12963], 
            [ 0.17778,  0.06667, -0.26667,  0.33333],
        ]);
        assert_eq!(a.inverse(), i);
    }
    #[test]
    fn mul_matrix_4x4_inverse() {
        let a = Matrix4x4::new([
            [ 3., -9.,  7.,  3.],
            [ 3., -8.,  2., -9.],
            [-4.,  4.,  4.,  1.],
            [-6.,  5., -1.,  1.],
        ]);
        let b = Matrix4x4::new([
            [ 8.,  2.,  2.,  2.],
            [ 3., -1.,  7.,  0.],
            [ 7.,  0.,  5.,  4.],
            [ 6., -2.,  0.,  5.],
        ]);
        let c = a * b; 
        assert_eq!(c * b.inverse(), a);
    }
    // Color tests.
    #[test]
    fn add_colors() {
        assert_eq!(
            Color::new(0.9, 0.6, 0.75) + Color::new(0.7, 0.1, 0.25),
            Color::new(1.6, 0.7, 1.0)
        )
    }
    #[test]
    fn sub_colors() {
        assert_eq!(
            Color::new(0.9, 0.6, 0.75) - Color::new(0.7, 0.1, 0.25),
            Color::new(0.2, 0.5, 0.5)
        )
    }
    #[test]
    fn scale_color() {
        assert_eq!(Color::new(0.2, 0.3, 0.4) * 2.0, Color::new(0.4, 0.6, 0.8))
    }
    #[test]
    fn mul_colors() {
        assert_eq!(
            Color::new(1.0, 0.2, 0.4) * Color::new(0.9, 1.0, 0.1),
            Color::new(0.9, 0.2, 0.04)
        )
    }
    // Canvas tests.
    #[test]
    fn set_pixel_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.set_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }
    #[test]
    fn write_blank_canvas() {
        let c = Canvas::new(10, 20);
        c.write_ppm("images/blank.ppm");
    }
}
