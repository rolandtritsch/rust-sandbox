extern crate rand;

use rand::Rng;

const N: i32 = 10;

struct Matrix {
    data: [[i32, .. N], .. N];
}

#[cfg(test)]
mod test {
    fn make_mat(n: i32) -> Matrix {
        let mut out = Matrix {[[0; N]; N]};

        for i in range(0, N) {
            for j in range(0, N) {
                out.data[i, j] = rand::thread_rng().gen_range(0, N);
            }
        }
        out
    }

    #[test]
    fn mul_test() {
        let a = make_mat(N);
        let b = make_mat(N);

        let c = mul(a, b);

        let mut check = 0;
        for i in range(0, N) {
            check += a[0, i] * b[i, 0];
        }

        assert_eq!(c.data[0, 0], check)
    }
}

mod matmul {
    pub fn mul(a: Matrix, b: Matrix) -> Matrix {
        let mut out = Matrix {[[0; N]; N]};

        for i in range(0, N) {
            for j in range(0, N) {
                for k in range(0, N) {
                    out.data[i][j] += a.data[i][k] * b.data[k][j];
                }
            }
        }
        out
    }
}
