
// Add_Student, Add_Course, Enroll, -> traits
// Grade, Search_Student, Filter_Student,
// List, Count, Report, Delete

// Traits, Structs, and Implementations to be used:
// TRAIT INHERITANCE: Identifiable -> Record -> Queryable
use std::io;
use std::fmt;
use std::collections::HashMap;

pub trait Identifiable{
    fn id(&self) -> u64; // return the unique ID of the record
    fn set_id(&mut self, id: u64); // set new ID for the record
}
pub trait Record:Identifiable{
    fn table_name() -> &'static str; // return table name
    fn validate(&self) ->Result<(), String>; // validate the record if the email has an @ sign
    fn to_record_string(&self) -> String; // create a display string
}
pub trait Queryable: Record{
    fn matches(&self, search_term: &str) -> bool; // check if the record matches search term
    fn matches_field(&self, field: &str, value: &str) -> bool; // check if field value match
}
pub trait Summarizable{
    fn summarize(&self) -> String; // return summary of the record
}
// implement summarizable as blanket impl for any type where T: Record + Queryable
impl <T> Summarizable for T
where T: Record + Queryable,
{
    fn summarize(&self) -> String {
        self.to_record_string()
    }
}
// STRUCTS:
#[derive(Clone)]
struct Student{
    id: u64,
    student_number: String,
    first_name: String,
    last_name: String,
    email: String,
    year_level: u8,
    is_active: bool
}
impl Student {
    fn new(
        student_number: String,
        first_name: String,
        last_name: String,
        email: String,
        year_level: u8,
    ) -> Self {
        Self {
            id: 0,
            student_number,
            first_name,
            last_name,
            email,
            year_level,
            is_active: true,
        }
    }
}
// impl blocks for the different traits:
impl Identifiable for Student{
    fn id(&self) -> u64{
        self.id
    }
    fn set_id(&mut self, id: u64){
        self.id = id;
    }
}
// table name, valdiate, to record string
impl Record for Student{
    fn table_name() -> &'static str{
        "STUDENTS"
    }
    fn validate(&self)->Result<(), String>{
        if !self.email.contains("@"){
            // if it does NOT contain the @
            return Err("INVALID EMAIL ADDRESS".to_string());
        }

        if self.year_level < 1 || self.year_level > 5{
            return Err("INVALID YEAR LEVEL".to_string());
        }
        Ok(()) // https://doc.rust-lang.org/std/result/ just asserting success
    }
    fn to_record_string(&self) -> String {
        format!("[{}] {}, {} (Year{}) - {}",
                self.student_number,
                self.last_name,
                self.first_name,
                self.year_level,
                self.is_active)
    }
}

impl Queryable for Student {
    fn matches(&self, search_term: &str) -> bool {
        let term = search_term.to_lowercase();
        // compare student number, first name, or last name (case insensitive kaya lower case)
        self.student_number.to_lowercase().contains(&term) ||
            self.first_name.to_lowercase().contains(&term) ||
            self.last_name.to_lowercase().contains(&term)
    }
    fn matches_field(&self, field: &str, value: &str) -> bool {
        match field {
            "year_level" => self.year_level.to_string() == value, // if field is exaclty the string, then compare to the value n return bool
            "is_active" => {
                match value {
                    "true" => self.is_active,
                    "false" => !self.is_active,
                    _ => false,
                }
            }
            _ => false, // anything else then wala
        }
    }
}

#[derive(Clone)]
struct Course{
    id: u64,
    course_code: String,
    course_name: String,
    units: u8,
    department: String
}
impl Identifiable for Course{
    fn id(&self) -> u64{
        self.id
    }
    fn set_id(&mut self, id: u64){
        self.id = id;
    }
}
impl Record for Course{
    fn table_name() -> &'static str{
        "COURSES"
    }
    fn validate(&self)->Result<(), String>{
        if self.course_code.is_empty(){
            return Err("COURSE_CODE CANNOT BE EMPTY".to_string());
        }
        if self.units < 1 || self.units > 6{
            return Err("UNITS NOT VALID".to_string());
        }
        Ok(())
    }
    fn to_record_string(&self) -> String {
        format!("[{}] {}, ({} units),- {})",
        self.course_code, self.course_name, self.units, self.department)
    }

}

#[derive(Clone)]
struct Enrollment<S: Clone + PartialEq>{
    id: u64,
    student_id: S,
    course_id: S,
    semester_type: Semester,
    grade: Option<f32>
}
impl<S: Clone + PartialEq + fmt::Display> Enrollment<S>{
    fn new(student_id: S, course_id: S, semester_type: Semester)->Self{
        Self{
            id:0,
            student_id,
            course_id,
            semester_type,
            grade: None,
        }
    }
    fn set_grade(&mut self, grade: f32){
        self.grade = Some(grade);
    }
    fn is_passing(&self)->bool{
        match self.grade{
            Some(upg) => upg >= 1.0 && upg <=3.0,
            None => false,
        }
    }
}

impl<S: Clone + PartialEq> Identifiable for Enrollment<S>{
    fn id(&self) -> u64{
        self.id
    }
    fn set_id(&mut self, id: u64){
        self.id = id;
    }
}

impl <S: Clone + PartialEq + fmt::Display> Record for Enrollment<S>{
    fn table_name() -> &'static str{
        "ENROLLMENTS"
    }
    fn validate(&self)->Result<(), String>{
        match self.semester_type{
            Semester::First(year) |
            Semester::Second(year)|
            Semester::Midyear(year) => {
                if year < 2020 || year > 2030 {
                    return Err("INVALID YEAR".to_string());
                }
            }
        }
        if let Some(upg) = self.grade{
            if upg < 0.0 || upg > 5.0 {
                return Err("INVALID UPG".to_string());
            }
        }
        Ok(())
    }
    fn to_record_string(&self) -> String {
        let grade_disp = match self.grade{
            Some(upg)=> format!("{:.2}", upg),
            None => "Ungraded".to_string(),
        };

        format!("[E{}] Student{} in Course {} ({}) - {}",
        self.id, self.student_id, self.course_id, self.semester_type, grade_disp)
    }
}

#[derive(Clone)]
struct Table<T: Record + Clone>{
    records: Vec<T>, // store records here
    next_id: u64, // prepare the next ID
}
impl<T: Record + Clone> Table<T>{
    fn new() -> Self {
        Self{
            records: Vec::new(),
            next_id: 1,
        }
    }
    fn insert(&mut self, mut record: T) -> Result<u64,String>{
        match record.validate(){
            Ok(_) => {}
            Err(msg) => return Err(msg),
        }
        let id = self.next_id;
        record.set_id(id);
        self.records.push(record);
        self.next_id += 1;
        Ok(id)
    }
    // other required functions:
    fn find_by_id(&self, id: u64)->Option<&T>{
        for record in &self.records {
            if record.id() == id {
                return Some(record);
            }
        }
        None
    }
    fn delete(&mut self, id: u64) -> Result<(),String>{
        let mut index = None;
        for i in 0..self.records.len() {
            if self.records[i].id() == id {
                index = Some(i);
                break;
            }
        }
        match index {
            Some(i) => {
                self.records.remove(i);
                Ok(())
            }
            None => Err("Record not found".to_string()),
        }
    }
    fn count(&self)->usize{
        self.records.len()
    }
    fn all(&self)->Vec<&T>{
        let mut vec = Vec::new();
        for record in &self.records {
            vec.push(record);
        }
        vec
    }
}
impl<T: Record + Clone + Queryable> Table<T>{
    fn filter_by(&self, field: &str, value: &str)->Vec<&T>{
        let mut vec = Vec::new();
        for record in &self.records {
            if record.matches_field(field, value){
            vec.push(record);
            }
        }
        vec
    }
}

#[derive(Clone)]
enum Semester{
    First(u32),
    Second(u32),
    Midyear(u32),
    // this is used in the semester type and year of struct Enrollment
}
impl fmt::Display for Semester {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Semester::First(year) => write!(f, "1st Sem {}", year),
            Semester::Second(year) => write!(f, "2nd Sem {}", year),
            Semester::Midyear(year) => write!(f, "Midyear {}", year),
        }
    }
}
// Generic Functions
fn print_all<T: Record>(records: Vec<T>){
    for i in 0..records.len(){
        println!("{}. {}", i + 1, records[i].to_record_string());
    }
}
fn count_where<T, F>(records: Vec<&T>, predicate: F)->usize
where
    F: Fn(&T) -> bool,
{
    let mut count = 0;
    for record in records {
        if predicate(&record) {
            count += 1;
        }
    }
    count
}

fn generate_report<T, F>(records: Vec<&T>, key_fn: F)
where
    T: Record,
    F: Fn(&T) -> String,
{
    let mut groups: HashMap<String, Vec<&T>> = HashMap::new();
    // group the records
    for record in records {
        let key = key_fn(record);
        if groups.contains_key(&key) {
            groups.get_mut(&key).unwrap().push(record);
        }
        else{
            groups.insert(key, vec![record]);
        }
    }
    for (key, group) in groups {
        println!("{}", key);
        for i in 0..group.len(){
            println!("{}. {}", i + 1, group[i].to_record_string());
        }
    }
}

fn main(){
    let mut students: Table<Student> = Table::new();
    let mut courses: Table<Course> = Table::new();
    let mut enrollments: Table<Enrollment<u64>> = Table::new();

    let mut first_line = String::new();
    io::stdin().read_line(&mut first_line).unwrap();

    let n: usize = first_line.trim().parse().unwrap();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty(){
            continue;
        }

        match parts[0]{
            "ADD_STUDENT" => {
                if parts.len() != 6 { // 6 token input essentially
                    println!("INVALID ADD_STUDENT");
                    continue;
                }
                let year_level: u8 = match parts[5].parse::<u8>(){
                    Ok(year) => year,
                    Err(_) => {
                        println!("INVALID YEAR LEVEL");
                        continue;
                    }
                };

                let student = Student::new(
                    parts[1].to_string(),
                    parts[2].to_string(),
                    parts[3].to_string(),
                    year_level,
                );
                match student.insert(student){
                    Ok(id) => println!("[STUDENTS] Record added with ID: {}", id),
                    Err(msg) => println!("Error: {}", msg),
                }
            }
            "ADD_COURSE" => {
                if parts.len() != 5 {
                    println!("INVALID ADD_COURSE");
                    continue;
                }
                let units: u8 = match parts[3].parse(){
                    Ok(unit) => unit,
                    Err(_) => {
                        println!("INVALID UNITS");
                        continue;
                    }
                };
                let course = Course{
                    id: 0,
                    course_code: parts[1].to_string(),
                    course_name: parts[2].to_string(),
                    units,
                    department: parts[4].to_string(),
                };
                match course.insert(course){
                    Ok(id) => println!("[COURSES] Record added with ID: {}", id),
                    Err(msg) => println!("Error: {}", msg),
                }

            }
            "ENROLL" => {
                if parts.len() != 5 {
                    println!("INVALID ENROLL");
                    continue;
                }
                let student_id: u64 = match parts[1].parse(){
                    Ok(student) => student,
                    Err(_) => continue,
                };
                let course_id: u64 = match parts[2].parse(){
                    Ok(course) => course,
                    Err(_) => continue,
                };
                let year: u8 = match parts[4].parse::<u8>(){
                    Ok(year) => year,
                    Err(_) => continue,
                };
                let semester = match parts[3]{
                    "1" => Semester::First(year),
                    "2" => Semester::Second(year),
                    "3" => Semester::Midyear(year),
                    _ => continue,
                };
                let enrollment = Enrollment::new(student_id, course_id, semester);
                match enrollment.insert(enrollment){
                    Ok(id) => println!("[ENROLL] Record added with ID: {}", id),
                    Err(msg) => println!("Error: {}", msg),
                }
            }
            "GRADE" => {
                if parts.len() != 3 {
                    println!("INVALID GRADE");
                    continue;
                }
                let enrollment_id: u64 = match parts[1].parse(){
                    Ok(enrollment) => enrollment,
                    Err(_) => continue,
                };
                let grade: u64 = match parts[2].parse(){
                    Ok(grade) => grade,
                    Err(_) => continue,
                };
                if let Some(enrollment) = enrollments
                    .records
                    .iter_mut()
                    .find(|record| record.id() == enrollment_id){
                    enrollment.set_grade(grade);
                }
            }
            "SEARCH_STUDENT" => {
                if parts.len() != 2 {
                    println!("INVALID SEARCH_STUDENT");
                    continue;
                }
                let results = students
                    .all()
                    .into_iter()
                    .filter(|record| record.id() == parts[1].id)
                    .collect::<Vec<&Student>>();
                println!(results);
            }
            "FILTER_STUDENTS" =>{
                if parts.len() != 3 {
                    continue;
                }
                let records = students.filter_by(parts[1], parts[2]);
                println!(records);

            }
            "LIST" => {
                if parts.len() != 2 {
                    continue;
                }
                match parts[1]{
                    "STUDENTS" => {
                        print_all(students.all());
                    }
                    "COURSES" => {
                        print_all(courses.all());
                    }
                    "ENROLLMENTS" => {
                        print_all(enrollments.all());
                    }
                    _ => {}
                }
            }
            "COUNT" => {
                if parts.len() < 2 {
                    continue;
                }
                match parts[1]{
                    "STUDENTS" => println!("{}", students.count()),
                    "ENROLLMENTS" => {
                        if parts.len() == 3 && parts[2] == "passing" {
                            let count = count_where(
                                enrollments.all(),
                                |record| record.is_passing(),
                            );
                            println!("{}",count);
                        }
                        else{
                            println!("{}", enrollments.count());
                        }
                    }
                    _ => {}
                }
            }
            // "REPORT" => {
            //
            // }
            "DELETE" => {
                if parts.len() != 3 {
                    continue;
                }
                let id: u64 = match parts[2].parse(){
                    Ok(i) => i,
                    Err(_) => continue,
                };
                match parts[1]{
                    "STUDENTS" => {
                        let _ = students.delete(id);
                    }
                    "COURSES" => {
                        let _ = courses.delete(id);
                    }
                    "ENROLLMENTS" => {
                        let _ = enrollments.delete(id);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

