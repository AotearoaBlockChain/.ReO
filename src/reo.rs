pub struct ReoScript {
    pub code: String,
}

impl ReoScript {
    pub fn new(code: &str) -> Self {
        ReoScript {
            code: code.to_string(),
        }
    }

    pub fn execute(&self) {
        if self.code.contains("analyze_data") {
            analyze_data();
        } else {
            println!("Kāore he mahi mō tēnei kōwae.");
        }
    }
}

fn analyze_data() {
    // Example data analysis function
    let data = vec![1, 2, 3, 4, 5];
    let sum: i32 = data.iter().sum();
    println!("Kua tatauria ngā raraunga: {}", sum);

    let mean = sum as f32 / data.len() as f32;
    println!("Ko te toharite: {}", mean);

    let max = data.iter().max().unwrap();
    println!("Ko te tino teitei: {}", max);
}
