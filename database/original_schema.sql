DROP TABLE IF EXISTS chapters;
DROP TABLE IF EXISTS questions;
DROP TABLE IF EXISTS students;
DROP TABLE IF EXISTS students_history;
DROP TABLE IF EXISTS courses;
DROP TRIGGER IF EXISTS student_update_trigger;
DROP TRIGGER IF EXISTS student_delete_trigger;

create table courses (
    course_id         int unsigned auto_increment primary key,
    name       varchar(255)                          not null,
    added_timestamp   timestamp default current_timestamp() null,
    updated_timestamp timestamp default current_timestamp() null on update current_timestamp(),
    description       varchar(1024) not null
);

create table chapters (
    chapter_id int unsigned primary key,
    course_id int unsigned,
    name varchar(128),
    description varchar(512)
);

-- create table questions (
--     question_id       int unsigned auto_increment primary key,
--     course_id         int unsigned                          not null,
--     chapter_id        int unsigned                          not null,
--     id_hash           varchar(32)                           not null,
--     q_text            varchar(2048)                         not null,
--     choices           varchar(2048) collate utf8mb4_bin     not null
--         check (json_valid(`choices`)),
--     answers           varchar(2048) collate utf8mb4_bin     not null
--         check (json_valid(`answers`)),
--     a_explanation     varchar(2048)                         not null,
--     a_hint            varchar(1024)                         not null,
--     difficulty        tinyint                               not null,
--     diff_reason       varchar(1024)                         not null,
--     added_timestamp   timestamp default current_timestamp() null,
--     updated_timestamp timestamp default current_timestamp() null on update current_timestamp(),
--     q_hash            varchar(2048)                           not null,
--     constraint questions_ibfk_1
--         foreign key (course_id) references courses (course_id)
-- );

-- create index idx_question_course on questions (course_id);

create table questions
(
    question_id       int unsigned auto_increment
        primary key,
    course_id         int unsigned                          not null,
    chapter_id        int unsigned                          not null,
    id_hash           varchar(32)                           not null,
    q_text            varchar(2048)                         not null,
    choices           varchar(2048) collate utf8mb4_bin     not null
        check (json_valid(`choices`)),
    answers           varchar(2048) collate utf8mb4_bin     not null
        check (json_valid(`answers`)),
    a_explanation     varchar(2048)                         not null,
    a_hint            varchar(1024)                         not null,
    difficulty      tinyint                               not null,
    diff_reason       varchar(1024)                         not null,
    added_timestamp   timestamp default current_timestamp() null,
    updated_timestamp timestamp default current_timestamp() null on update current_timestamp(),
    q_hash            varchar(2048)                         null,
    constraint questions_q_hash_uindex
        unique (q_hash) using hash,
    constraint questions_ibfk_1
        foreign key (course_id) references courses (course_id)
);

create index idx_question_course
    on questions (course_id);




create table students (
    student_id        int unsigned auto_increment primary key,
    id_hash           varchar(255)                          not null,
    first_name        varchar(255)                          not null,
    email_address     varchar(255)                          not null,
    email_hash        varchar(255)                          not null,
    pass_hash          varchar(255)                          not null,
    over_13           bit                                   not null,
    email_verified    bit                                   not null,
    account_active    bit                                   not null,
    added_timestamp   timestamp default current_timestamp() null,
    constraint email_address unique (email_address)
);

create index idx_student_email on students (email_address);

create index students_email_address_hash_index on students (email_hash);

create index students_student_id_hash_index on students (id_hash);

create table students_history (
    history_id int unsigned auto_increment primary key,
    student_id int unsigned,
    action_type enum('update', 'delete'),
    timestamp timestamp default current_timestamp(),
    id_hash varchar(255) not null,
    first_name        varchar(255)                          not null,
    email_address     varchar(255)                          not null,
    email_hash        varchar(255)                          not null,
    pass_hash          varchar(255)                          not null,
    over_13           bit                                   not null,
    email_verified    bit                                   not null,
    account_active    bit                                   not null
);

CREATE TRIGGER student_update_trigger
    AFTER UPDATE ON students
    FOR EACH ROW
BEGIN
    INSERT INTO students_history (student_id, action_type, id_hash, first_name, email_address, email_hash, pass_hash, over_13, email_verified, account_active)
    VALUES (old.student_id, 'update', old.id_hash, old.first_name, old.email_address, old.email_hash, old.pass_hash, old.over_13, old.email_verified, old.account_active);
END;

CREATE TRIGGER student_delete_trigger
    AFTER DELETE ON students
    FOR EACH ROW
BEGIN
    INSERT INTO students_history (student_id, action_type, id_hash, first_name, email_address, email_hash, pass_hash, over_13, email_verified, account_active)
    VALUES (old.student_id, 'delete', old.id_hash, old.first_name, old.email_address, old.email_hash, old.pass_hash, old.over_13, old.email_verified, old.account_active);
END;


insert into courses(course_id, name, description)
values(1, 'AP Computer Science Principles', "AP Computer Science Principles is an elective course offered by the College Board's Advanced Placement (AP) program, providing high school students worldwide with an introductory college-level exploration of foundational computer science concepts. Through this course, students delve into computational thinking, problem-solving skills, and various aspects of computing, including programming, algorithms, data analysis, internet fundamentals, cybersecurity, and the societal impacts of technology. While typically not mandatory for graduation, students have the option to select this course to gain a deeper understanding of computer science principles and their applications.");

insert into courses(course_id, name, description)
values(2, 'AP Computer Science A', "AP Computer Science A is an elective course offered by the College Board's Advanced Placement (AP) program, catering to high school students globally seeking an in-depth study of computer science fundamentals. Through this course, students delve into programming concepts using the Java programming language, focusing on topics such as algorithms, data structures, software development methodologies, and object-oriented programming principles. While not typically mandatory for graduation, students have the opportunity to choose this course to develop their programming skills and prepare for advanced study or careers in computer science and related fields.");

insert into chapters(chapter_id, course_id, name, description)
values(1, 1, "Creative Development", "This section focuses on the creative process behind developing computational artifacts. By exploring collaboration, program function, design, development, and error correction, students learn to create effective software solutions that meet user needs. This fosters creativity, problem-solving skills, and an understanding of how to debug and improve software, preparing students for further study or careers in software development and engineering.");

insert into chapters(chapter_id, course_id, name, description)
values(2, 1, "Data", "Students delve into the representation, transformation, and analysis of data. Through topics like binary numbers, data compression, and data manipulation with programs, they gain critical insights into how data underpins all computing systems and applications. This knowledge is crucial for understanding the digital world, enhancing students' abilities to make data-driven decisions and to process and interpret vast amounts of information efficiently.");

insert into chapters(chapter_id, course_id, name, description)
values(3, 1, "Algorithms and Programming", "Covering the fundamentals of programming and algorithmic thinking, this comprehensive section includes variables, data abstraction, control structures, lists, and more. Students learn to design algorithms to solve problems and to implement those algorithms in software. This equips them with the problem-solving skills and technical knowledge necessary for software development, computational thinking, and the pursuit of advanced computer science studies.");

insert into chapters(chapter_id, course_id, name, description)
values(4, 1, "Computer Systems and Networks", "By understanding the internet, fault tolerance, and the basics of parallel and distributed computing, students grasp the foundational concepts that allow global connectivity and data exchange. This understanding is essential for recognizing the role of computer systems in facilitating communication, ensuring data reliability, and supporting the distributed nature of modern computing.");

insert into chapters(chapter_id, course_id, name, description)
values(5, 1, "Impact of Computing", "This idea explores the societal impacts of computing, including its benefits, the digital divide, bias, crowdsourcing, legal and ethical concerns, and safe computing practices. It prepares students to be informed citizens who can critically assess the implications of technology on society and contribute positively to discussions about technology's role in addressing social, ethical, and legal issues.");

insert into students(student_id, id_hash, first_name, email_address, email_hash, pass_hash, over_13, email_verified, account_active)
values(1, '83dcefb7','John','jon@abc.com', '29599b12', '$2b$12$kXgej2NgVd6RJu9WxjiBS.E57vHpQrMqm7Cg9rY9LvosTyisKuwHS',true,false,false);

insert into students(student_id, id_hash, first_name, email_address, email_hash, pass_hash, over_13, email_verified, account_active)
values(2, '1ad5be0d','Jane','jane@abc.com', 'b5d81a5f', '$2b$12$3hMwKn9QM4Zc2.t3/pX9PupgD.fjryIrCnonUlgKIwjtxxfwIhm4i',true,false,false);

insert into students(student_id, id_hash, first_name, email_address, email_hash, pass_hash, over_13, email_verified, account_active)
values(3, '6dd28e9b','Joe','joe@abc.com', 'ab0c05cf', '$2b$12$VVeWdlCP9u3pthuClIYWluOeoUyXt8BreRBqk42U49ynaUZ54R9ru',true,false,false);
