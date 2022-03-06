// use std::cell::RefCell;
// use std::rc::Rc;

// #[allow(dead_code)]
// #[allow(dead_code)]
// struct Student {
//     name: String,
//     courses: Vec<Rc<RefCell<Course>>>,
// }

// impl Student {
//     fn new(name: &str) -> Self {
//         Self {
//             name: name.into(),
//             courses: Vec::new(),
//         }
//     }
// }

// #[allow(dead_code)]
// struct Course {
//     name: String,
//     students: Vec<Rc<RefCell<Student>>>,
// }

// impl Course {
//     fn new(name: &str) -> Self {
//         Self {
//             name: name.into(),
//             students: Vec::new(),
//         }
//     }

//     fn add_student(course: Rc<RefCell<Course>>, student: Rc<RefCell<Student>>) {
//         student.borrow_mut().courses.push(course.clone());
//         course.borrow_mut().students.push(student);
//     }
// }

// fn main() {
//     println!("-------------Circular Reference-------------");

//     let prince = Rc::new(RefCell::new(Student::new("Prince")));
//     let tyrion = Rc::new(RefCell::new(Student::new("Tyrion")));

//     let course = Course::new("Rust Course");
//     let magic_course = Rc::new(RefCell::new(course));

//     Course::add_student(magic_course.clone(), prince);
//     Course::add_student(magic_course, tyrion);
// }

#[allow(dead_code)]
struct Student {
    name: String,
}

#[allow(dead_code)]
impl Student {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform
            .enrollments
            .iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

#[allow(dead_code)]
struct Course {
    name: String,
}

#[allow(dead_code)]
struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course,
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Self {
        Self { student, course }
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>,
}

impl<'a> Platform<'a> {
    fn new() -> Self {
        Self {
            enrollments: Vec::new(),
        }
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(Enrollment::new(student, course));
    }
}

fn main() {
    println!("-------------Circular Reference-------------");

    let prince = Student {
        name: "Prince".into(),
    };

    let course = Course {
        name: "Intro To Rust".into(),
    };

    let mut p = Platform::new();
    p.enroll(&prince, &course);

    for c in prince.courses(p) {
        println!("Prince is taking {}", c);
    }
}
