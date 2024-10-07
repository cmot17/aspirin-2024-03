#[derive(PartialEq, Clone, Copy, Debug)]
enum ClassYear {
    Senior,
    Junior,
    Sophomore,
    FirstYear,
}

impl ClassYear {
    fn all() -> [Self; 4] {
        [Self::Senior, Self::Junior, Self::Sophomore, Self::FirstYear]
    }
}

struct Student {
    name: &'static str,
    class_year: ClassYear,
    gpa: f32,
}

const OLIN_STUDENTS: [Student; 8] = [
    Student {
        name: "Alice",
        class_year: ClassYear::Senior,
        gpa: 3.9,
    },
    Student {
        name: "Foo",
        class_year: ClassYear::Sophomore,
        gpa: 2.3,
    },
    Student {
        name: "Bar",
        class_year: ClassYear::Junior,
        gpa: 3.9,
    },
    Student {
        name: "Ralph",
        class_year: ClassYear::Senior,
        gpa: 3.1,
    },
    Student {
        name: "Ayush",
        class_year: ClassYear::Senior,
        gpa: 0.0,
    },
    Student {
        name: "Anna",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Hannah",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Lorin",
        class_year: ClassYear::Junior,
        gpa: 3.6,
    },
];

fn get_average_gpa() -> f32 {
    let mut num = 0;
    let mut sum: f32 = 0.0;
    for student in OLIN_STUDENTS {
        if student.class_year != ClassYear::FirstYear {
            sum += student.gpa;
            num += 1;
        }
    }
    sum / num as f32
}

fn get_num_excel_students_for_class(class_year: ClassYear) -> u32 {
    let mut num: u32 = 0;
    for student in OLIN_STUDENTS {
        if student.class_year == class_year && student.gpa > get_average_gpa() {
            num += 1;
        }
    }
    num
}

fn get_best_class() -> ClassYear {
    let mut best_class = (ClassYear::Senior, 0);
    for class in ClassYear::all() {
        let num = get_num_excel_students_for_class(class);
        if num > best_class.1 {
            best_class = (class, num)
        }
    }
    best_class.0
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::university::{
        get_average_gpa, get_best_class, get_num_excel_students_for_class, ClassYear,
    };

    #[test]
    fn test_get_average_gpa() {
        assert!(approx_eq!(f32, get_average_gpa(), 2.8))
    }

    #[test]
    fn test_get_num_excel_students_for_class() {
        assert_eq!(get_num_excel_students_for_class(ClassYear::Sophomore), 0);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Junior), 2);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Senior), 2);
    }

    #[test]
    fn test_get_best_class() {
        assert_eq!(get_best_class(), ClassYear::Senior);
    }
}
