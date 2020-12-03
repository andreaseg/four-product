use ndarray::Array2;

trait Join {
    fn join(self, joiner: &str) -> String;
}

impl<I: Iterator<Item = String>> Join for I {
    fn join(mut self, joiner: &str) -> String {
        let first = self.next().unwrap_or_else(|| return "".to_string());
        self.fold(first, |l, r| l + joiner + &r)
    }
}

pub fn pretty_print(matrix: &Array2<i32>) {
    let message: String = (0..matrix.nrows())
        .map(|x| {
            (0..matrix.ncols())
                .map(|y| format!("{:02}", matrix[[x, y]]))
                .join(" ")
        })
        .join("\n");

    println!("{}", message);
}
