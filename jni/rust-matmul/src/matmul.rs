extern crate libc;
extern crate rand;

use libc::{c_void};

const N: usize = 10;

#[repr(C)]
pub struct JNINativeInterface {
    reserved0: *mut c_void,
    reserved1: *mut c_void,
    reserved2: *mut c_void,
    reserved3: *mut c_void,
    // much more actually in here for practical JNI code, but not
    // relevant for this very simple example...
}

pub type JNIEnv = *const JNINativeInterface;

pub struct Matrix {
    data: [[i32; N]; N]
}

//#[no_mangle]
//#[allow(non_snake_case)]
//#[allow(unused_variables)]
//pub extern fn Java_MatMul_00024_matmul (jre: *mut JNIEnv, class: *const c_void, jobjectArray, jobjectArray, jobjectArray);

mod matmul {
    use ::Matrix;
    use ::N;

    pub fn mul(a: &Matrix, b: &Matrix, out: &mut Matrix) {
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    out.data[i][j] += a.data[i][k] * b.data[k][j];
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use matmul;
    use ::Matrix;
    use ::N;
    use rand::{thread_rng, Rng};

    fn make_mat(m: &mut Matrix) {
        for i in 0..N {
            for j in 0..N {
                m.data[i][j] = thread_rng().gen_range(0, (N as i32));
            }
        }
    }

    fn print_mat(m: &Matrix) {
        for i in 0..N {
            for j in 0..N {
                print!("{} ", m.data[i][j]);
            }
            println!("");
        }
    }

    #[test]
    fn mul_test() {
        let mut a = Matrix {data: [[0; N]; N]}; make_mat(&mut a);
        let mut b = Matrix {data: [[0; N]; N]}; make_mat(&mut b);
        let mut out = Matrix {data: [[0; N]; N]};

        matmul::mul(&a, &b, &mut out);

        let mut check = 0;
        for i in 0..N {
            check += a.data[0][i] * b.data[i][0];
        }

        print_mat(&a); println!("");
        print_mat(&b); println!("");
        print_mat(&out); println!("");
        print!("{}", check); println!("");

        assert_eq!(out.data[0][0], check)
    }
}
