
// In class Assignment

// Create a struct Student (major)
struct Student {
    major:String,
}

// Higher order functions update majors
fn update_majors(mut collection: Vec<Student>, behavior:fn(&mut Student,String)) -> Vec<Student> {
    let new_major = "CS".to_string();
    for student in collection.iter_mut() {
        behavior(student, new_major.clone());
    }
    collection
}

// First Order functions, assign_major(student,major_declared)
fn assign_major(s: &mut Student,major:String){
    s.major = major;
}


fn main(){
    // create a vector of students1,2,3 and update all students major 
    let students = vec![
        Student {major: "Undeclared".to_string()},
        Student {major: "Undeclared".to_string()},
        Student {major: "Undeclared".to_string()},
    ];

    let updated_students = update_majors(students,assign_major);

    for(i,student) in updated_students.iter().enumerate(){
    println!("Student {} major: {}", i + 1, student.major);
    }
}


