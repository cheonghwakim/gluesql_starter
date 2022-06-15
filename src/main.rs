use gluesql::prelude::*;

fn main() {
    let storage = SledStorage::new("data/doc-db").unwrap();
    let mut glue = Glue::new(storage);
    let sqls = vec![
        "DROP TABLE IF EXISTS Student;",
        "DROP TABLE IF EXISTS Score;",
        "CREATE TABLE Student (id INTEGER, name TEXT);",
        "CREATE TABLE Score (id INTEGER, subject TEXT, score DECIMAL);",
        "INSERT INTO Student VALUES (1, '김철수'), (2, '김영희');",
        "INSERT INTO Score VALUES (1, '수학', 95.6), (1, '영어', 51.3);",
        "SELECT st.id, st.name, sc.subject, sc.score 
         FROM Student st
         JOIN Score sc
         ON st.id = sc.id
         WHERE st.id = 1;",
    ];

    for sql in sqls {
        let output = glue.execute(sql).unwrap();
        println!("{:?}", output)
    }
}