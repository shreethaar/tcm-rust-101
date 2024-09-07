#![allow(unused)]

fn main() {
    //compound types - tuples & arrays
    //max value of tuple - 12
    let student_a = ("Heath",'A',3.58);
    let name_student_a = student_a.0;
    let grade_student_a = student_a.1;
    let gpa_student_a = student_a.2;
    // let(name_student_a, grade_student_a, gpa_student_a) = student_a;

    println!("My name is {}, my class grade is {}, my overall GPA is {}",name_student_a,grade_student_a,gpa_student_a);

    //arrays
    // use [] - store up to 32 - similar data types
    let students = ["Heath","Bob","Linda"];
    println!("The first student in our array is {}", students[0]);
    println!("The second student in our array is {}", students[1]);
    println!("The third student in our array is {}", students[2]);

}
