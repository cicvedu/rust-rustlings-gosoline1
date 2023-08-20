// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.




pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: std::fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

// 为 ReportCard<T> 实现 From<f32> 特质，将 f32 类型的数字成绩转换为字符串形式
impl From<f32> for ReportCard<String> {
    fn from(grade: f32) -> Self {
        ReportCard {
            grade: format!("{:.1}", grade), // 将数字格式化为一个小数位
            student_name: "".to_string(),
            student_age: 0,
        }
    }
}

// 为 ReportCard<T> 实现 From<&str> 特质，将字符串成绩直接转换为 ReportCard
impl<'a> From<&'a str> for ReportCard<&'a str> {
    fn from(grade: &'a str) -> Self {
        ReportCard {
            grade,
            student_name: "".to_string(),
            student_age: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card: ReportCard<String> = 2.1.into(); // 使用 From<f32> 特质转换
        assert_eq!(
            report_card.print(),
            " (0) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card: ReportCard<&str> = "A+".into(); // 使用 From<&str> 特质转换
        assert_eq!(
            report_card.print(),
            " (0) - achieved a grade of A+"
        );
    }
}
