use std::ops::Deref;

#[derive(Debug)]
pub struct SquareMat {
    data: Vec<i32>,
    pub size: usize,
}

impl SquareMat {
    pub fn get(&self, row: usize, col: usize) -> i32 {
        self.data
            .get(self.size * row + col)
            .copied()
            .expect("be careful")
    }

    fn new(data: Vec<i32>) -> Result<Self, String> {
        let len = data.len();
        let size = len.isqrt();

        if size.pow(2) != len {
            return Err(format!("{data:?} has an invalid size"));
        }

        Ok(Self { data, size })
    }

    fn square(&self) -> Self {
        let size = self.size;
        let mut data: Vec<i32> = (0..size.pow(2)).map(|_| 0).collect();

        for row in 0..size {
            for col in 0..size {
                for i in 0..size {
                    data[size * row + col] += self.get(row, i) * self.get(i, col);
                }
            }
        }

        Self { data, size }
    }

    fn is_id(&self) -> bool {
        for row in 0..self.size {
            for col in 0..self.size {
                let is_diag = row == col;

                match self.get(row, col) {
                    1 if is_diag => {}
                    0 if !is_diag => {}
                    _ => return false,
                }
            }
        }

        true
    }
}

#[derive(Debug)]
pub struct InvlMat {
    mat: SquareMat,
}

impl InvlMat {
    pub fn new(data: Vec<i32>) -> Result<Self, String> {
        let mat = SquareMat::new(data)?;
        if mat.square().is_id() {
            Ok(Self { mat })
        } else {
            Err(format!("{:?} is not involutory", mat.data))
        }
    }
}

impl Deref for InvlMat {
    type Target = SquareMat;

    fn deref(&self) -> &Self::Target {
        &self.mat
    }
}
