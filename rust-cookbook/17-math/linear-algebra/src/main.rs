#[macro_use(array)]
extern crate ndarray;
use ndarray::Array;
use ndarray::{arr2, Array1, ArrayView1};
use nalgebra::Matrix3;
fn l1_norm(x: ArrayView1<f64>) -> f64 {
    x.fold(0., |acc, elem| acc + elem.abs())
}

fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e / norm);
    x
}
fn main() {
    let a = Array::from_vec(vec![1., 2., 3., 4., 5.]);
    let b = Array::from_vec(vec![5., 4., 3., 2., 1.]);
    let mut c = Array::from_vec(vec![1., 2., 3., 4., 5.]);
    let mut d = Array::from_vec(vec![5., 4., 3., 2., 1.]);

    let z = a + b;
    let w = &c + &d;

    let epsilon = 1e-8;
    for elem in z.iter() {
        let diff: f32 = *elem - 6.;
        assert!(diff.abs() < epsilon);
    }

    println!("c = {}", c);
    c[0] = 10.;
    d[1] = 10.;

    for elem in w.iter() {
        let diff: f32 = *elem - 6.;
        assert!(diff.abs() < epsilon);
    }

    let x = array![1., 2., 3., 4., 5.];
    println!("||x||_2 = {}", l2_norm(x.view()));
    println!("||x||_1 = {}", l1_norm(x.view()));
    println!("Normalizing x yields {:?}", normalize(x));

    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);

    let b = arr2(&[[6, 5, 4], [3, 2, 1]]);

    println!("Sum: {}", a + b);

    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);

    let b = arr2(&[[6, 3], [5, 2], [4, 1]]);

    println!("{}", a.dot(&b));

    let m1 = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
    println!("m1 = {}", m1);
    match m1.try_inverse() {
        Some(inv) => {
            println!("The inverse of m1 is: {}", inv);
        }
        None => {
            println!("m1 is not invertible!");
        }
    }
}
