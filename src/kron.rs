use arrayfire::{assign_seq, index, Array, Dim4, DType, Seq};

fn get(a: &Array, i: i32, j: i32) -> Array {
    let seqs = &[Seq::new(i, i, 1), Seq::new(j, j, 1)];
    return index(a, seqs);
}

pub fn kron(a: &Array, b: &Array) -> Array {
    let a_rows = a.dims().get()[0] as i32;
    let a_cols = a.dims().get()[1] as i32;

    let b_rows = b.dims().get()[0] as i32;
    let b_cols = b.dims().get()[1] as i32;

    let new_dims = Dim4::new(&[(a_rows * b_rows) as u64, (a_cols * b_cols) as u64, 1, 1]);
    let mut new_array = Array::new_empty(new_dims, DType::C32);

    for i in 0..a_rows {
        for j in 0..a_cols {
            let current_mat = b * get(a,i,j);

            let seqs = &[Seq::new(i * b_rows, (i + 1) * b_rows - 1, 1), Seq::new(j * b_cols, (j + 1) * b_cols - 1, 1)];
            new_array  = assign_seq(&new_array, seqs, &current_mat);
        }
    }

    return new_array
}


