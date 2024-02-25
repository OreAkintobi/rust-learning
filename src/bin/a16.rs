// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    // * Print out the details of a student's locker assignment
    // * Lockers use numbers and are optional for students
    let student_1 = Student {
        name: "David".to_owned(),
        locker: Some(123),
    };
    let student_2 = Student {
        name: "Robert".to_owned(),
        locker: None,
    };

    fn print_locker(student: &Student) {
        match student.locker {
            Some(locker) => println!("Student: {:?}, Locker: {:?}", student.name, locker),
            None => println!(
                "Student: {:?}, Locker: {:?}",
                student.name, "No locker assigned"
            ),
        }
    }

    print_locker(&student_1);
    print_locker(&student_2);
}
