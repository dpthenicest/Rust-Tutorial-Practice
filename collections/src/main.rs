fn main() {
    let mut students = vec![Student {
        name: "Ryan".to_string(),
    }];
    students.push(Student {
        name: "Lisa".to_string(),
    });

    assert!(&students[0] == &Student {
        name: "Ryan".to_string()
    });

    assert!(students.get(0) == Some(&Student {
        name: "Ryan".to_string()
    }));

    assert!(students.get(10000) == None);

    for student in students.iter(){
        println!("Student name: {}", student.name);
    }

    use std::collections::HashMap;

    let mut enrollment = HashMap::new();
    enrollment.insert("biology".to_string(), students);

    let bio_students = enrollment.get("biology");
    let students = enrollment.remove("biology");
}

#[derive(PartialEq, Eq)]
struct Student {
    name: String,
}

/* 
    Collections 
    1. Vec<T>: for accessing by index
    2. HashMap<K, V>: associates keys and values
    3. HashSet<T>: used in cases of unique items and where you do not need to access items by index 
    4. VecDeque<T>: used for modelling queue where you'd want to put items from one side and pop items from the other.
    5. LinkedList<T>

    Vec Memory: 
    let v = vec![1u6, 2, 3];
    It has three parts: length, capacity and data pointers.

    Slices: are references to items that leave continuous chunks of memory. They refer to data owned by another value.
    Written as &[T].
    let s = &v[0..2];
    It has two points: length and data pointers.
*/
    