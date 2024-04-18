drop database if exists aarya_v1;
create database aarya_v1;
use aarya_v1;

DROP TABLE IF EXISTS chapters;
DROP TABLE IF EXISTS questions;
DROP TABLE IF EXISTS students;
DROP TABLE IF EXISTS students_history;
DROP TABLE IF EXISTS courses;
DROP TABLE IF EXISTS tests;
DROP TABLE IF EXISTS test_chapters;
DROP TABLE IF EXISTS test_questions;
DROP TRIGGER IF EXISTS student_update_trigger;
DROP TRIGGER IF EXISTS student_delete_trigger;

create table courses (
    course_id         int unsigned auto_increment primary key,
    name       varchar(255)                          not null,
    id_hash           varchar(32)                           not null,
    added_timestamp   timestamp default current_timestamp() null,
    updated_timestamp timestamp default current_timestamp() null on update current_timestamp(),
    description       varchar(1024) not null
);

create table chapters (
    chapter_id int unsigned primary key,
    id_hash           varchar(32)                           not null,
    course_id int unsigned,
    name varchar(128),
    description varchar(512)
);

create table questions
(
    question_id       int unsigned auto_increment primary key,
    course_id         int unsigned                          not null,
    chapter_id        int unsigned                          not null,
    id_hash           varchar(32)                           not null,
    que_text            varchar(2048)                         not null,
    que_description            varchar(2048)                         not null,
    choices varchar(2048) collate utf8mb4_bin not null check (json_valid(`choices`)), -- [{"id":"abc", "text":"abc"}, {"id":"def", "text":"def"}]
    answers varchar(2048) collate utf8mb4_bin not null check (json_valid(`answers`)), -- [{"id":"abc"}]
    ans_explanation     varchar(2048)                         not null,
    ans_hint            varchar(1024)                         not null,
    difficulty      tinyint                               not null,
    diff_reason       varchar(1024)                         not null,
    added_timestamp   timestamp default current_timestamp() null,
    updated_timestamp timestamp default current_timestamp() null on update current_timestamp(),
    que_hash            varchar(2048)                         null,
    constraint questions_q_hash_uindex unique (que_hash) using hash,
    constraint questions_ibfk_1 foreign key (course_id) references courses (course_id)
);

create index idx_question_course
    on questions (course_id);


create table tests(
    test_id int unsigned auto_increment primary key,
    id_hash varchar(32) not null,
    name varchar(128) not null,
    kind tinyint not null, -- exam (1) or practice (0)
    course_id int unsigned not null,
    added_timestamp timestamp default current_timestamp(),
    description varchar(1024) not null,
    constraint uk_tests_id_hash unique (id_hash) using hash,
    constraint fk_tests_courses foreign key (course_id) references courses (course_id)
);

create table test_chapters(
    test_id int unsigned not null,
    chapter_id int unsigned not null,
    constraint fk_test_chapters_tests foreign key (test_id) references tests (test_id),
    constraint fk_test_chapters_chapters foreign key (chapter_id) references chapters (chapter_id)
);

create table test_questions(
    test_id int unsigned not null,
    question_id int unsigned not null,
    constraint fk_test_questions_tests foreign key (test_id) references tests (test_id),
    constraint fk_test_questions_questions foreign key (question_id) references questions (question_id)
);

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


insert into courses(course_id, id_hash, name, description)
values(1, "course-hash1", 'AP Computer Science Principles', "AP Computer Science Principles is an elective course offered by the College Board's Advanced Placement (AP) program, providing high school students worldwide with an introductory college-level exploration of foundational computer science concepts. Through this course, students delve into computational thinking, problem-solving skills, and various aspects of computing, including programming, algorithms, data analysis, internet fundamentals, cybersecurity, and the societal impacts of technology. While typically not mandatory for graduation, students have the option to select this course to gain a deeper understanding of computer science principles and their applications.");

insert into courses(course_id, id_hash, name, description)
values(2, "course-hash2", 'AP Computer Science A', "AP Computer Science A is an elective course offered by the College Board's Advanced Placement (AP) program, catering to high school students globally seeking an in-depth study of computer science fundamentals. Through this course, students delve into programming concepts using the Java programming language, focusing on topics such as algorithms, data structures, software development methodologies, and object-oriented programming principles. While not typically mandatory for graduation, students have the opportunity to choose this course to develop their programming skills and prepare for advanced study or careers in computer science and related fields.");

insert into chapters(chapter_id, id_hash, course_id, name, description)
values(1, "chapter-hash1", 1, "Creative Development", "This section focuses on the creative process behind developing computational artifacts. By exploring collaboration, program function, design, development, and error correction, students learn to create effective software solutions that meet user needs. This fosters creativity, problem-solving skills, and an understanding of how to debug and improve software, preparing students for further study or careers in software development and engineering.");

insert into chapters(chapter_id, id_hash, course_id, name, description)
values(2, "chapter-hash2", 1, "Data", "Students delve into the representation, transformation, and analysis of data. Through topics like binary numbers, data compression, and data manipulation with programs, they gain critical insights into how data underpins all computing systems and applications. This knowledge is crucial for understanding the digital world, enhancing students' abilities to make data-driven decisions and to process and interpret vast amounts of information efficiently.");

insert into chapters(chapter_id, id_hash, course_id, name, description)
values(3, "chapter-hash3", 1, "Algorithms and Programming", "Covering the fundamentals of programming and algorithmic thinking, this comprehensive section includes variables, data abstraction, control structures, lists, and more. Students learn to design algorithms to solve problems and to implement those algorithms in software. This equips them with the problem-solving skills and technical knowledge necessary for software development, computational thinking, and the pursuit of advanced computer science studies.");

insert into chapters(chapter_id, id_hash, course_id, name, description)
values(4, "chapter-hash3", 1, "Computer Systems and Networks", "By understanding the internet, fault tolerance, and the basics of parallel and distributed computing, students grasp the foundational concepts that allow global connectivity and data exchange. This understanding is essential for recognizing the role of computer systems in facilitating communication, ensuring data reliability, and supporting the distributed nature of modern computing.");

insert into chapters(chapter_id, id_hash, course_id, name, description)
values(5, "chapter-hash4", 1, "Impact of Computing", "This idea explores the societal impacts of computing, including its benefits, the digital divide, bias, crowdsourcing, legal and ethical concerns, and safe computing practices. It prepares students to be informed citizens who can critically assess the implications of technology on society and contribute positively to discussions about technology's role in addressing social, ethical, and legal issues.");

insert into students(student_id, id_hash, first_name, email_address, email_hash, pass_hash, over_13, email_verified, account_active)
values(1, '83dcefb7','John','jon@abc.com', '29599b12', '$2b$12$kXgej2NgVd6RJu9WxjiBS.E57vHpQrMqm7Cg9rY9LvosTyisKuwHS',true,false,false);

insert into students(student_id, id_hash, first_name, email_address, email_hash, pass_hash, over_13, email_verified, account_active)
values(2, '1ad5be0d','Jane','jane@abc.com', 'b5d81a5f', '$2b$12$3hMwKn9QM4Zc2.t3/pX9PupgD.fjryIrCnonUlgKIwjtxxfwIhm4i',true,false,false);

insert into students(student_id, id_hash, first_name, email_address, email_hash, pass_hash, over_13, email_verified, account_active)
values(3, '6dd28e9b','Joe','joe@abc.com', 'ab0c05cf', '$2b$12$VVeWdlCP9u3pthuClIYWluOeoUyXt8BreRBqk42U49ynaUZ54R9ru',true,false,false);


-- Seed data for tests table
INSERT INTO tests (id_hash, name, kind, course_id, description)
VALUES
    ('hash1', 'Test 1', 1, 1, 'Description for Test 1'),
    ('hash2', 'Test 2', 1, 1, 'Description for Test 2'),
    ('hash3', 'Test 3', 1, 1, 'Description for Test 3'),
    ('hash4', 'Test 4', 1, 1, 'Description for Test 4'),
    ('hash5', 'Test 5', 1, 1, 'Description for Test 5');

-- Seed data for test_chapters table
INSERT INTO test_chapters (test_id, chapter_id)
VALUES
    (1, 1), -- Test 1, Chapter 1
    (1, 2), -- Test 1, Chapter 2
    (2, 2), -- Test 2, Chapter 2
    (2, 3), -- Test 2, Chapter 3
    (3, 3), -- Test 3, Chapter 3
    (3, 4), -- Test 3, Chapter 4
    (4, 4), -- Test 4, Chapter 4
    (4, 5), -- Test 4, Chapter 5
    (5, 5); -- Test 5, Chapter 5

-- Insert statement 1
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 1, 'hash1', 'Question 1 text', 'Question 1 description', '[{"id":"abc", "text":"abc"}, {"id":"def", "text":"def"}]', '[{"id":"abc"}]', 'Explanation for Question 1', 'Hint for Question 1', 1, 'Reason for difficulty 1');

-- Insert statement 2
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 2, 'hash2', 'Question 2 text', 'Question 2 description', '[{"id":"ghi", "text":"ghi"}, {"id":"jkl", "text":"jkl"}]', '[{"id":"ghi"}]', 'Explanation for Question 2', 'Hint for Question 2', 2, 'Reason for difficulty 2');

-- Insert statement 3
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 3, 'hash3', 'Question 3 text', 'Question 3 description', '[{"id":"mno", "text":"mno"}, {"id":"pqr", "text":"pqr"}]', '[{"id":"mno"}]', 'Explanation for Question 3', 'Hint for Question 3', 3, 'Reason for difficulty 3');

-- Insert statement 4
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 4, 'hash4', 'Question 4 text', 'Question 4 description', '[{"id":"stu", "text":"stu"}, {"id":"vwx", "text":"vwx"}]', '[{"id":"stu"}]', 'Explanation for Question 4', 'Hint for Question 4', 4, 'Reason for difficulty 4');

-- Insert statement 5
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 5, 'hash5', 'Question 5 text', 'Question 5 description', '[{"id":"yz1", "text":"yz1"}, {"id":"234", "text":"234"}]', '[{"id":"yz1"}]', 'Explanation for Question 5', 'Hint for Question 5', 5, 'Reason for difficulty 5');

-- Insert statement 6
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 6, 'hash6', 'Question 6 text', 'Question 6 description', '[{"id":"567", "text":"567"}, {"id":"890", "text":"890"}]', '[{"id":"567"}]', 'Explanation for Question 6', 'Hint for Question 6', 1, 'Reason for difficulty 1');

-- Insert statement 7
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 7, 'hash7', 'Question 7 text', 'Question 7 description', '[{"id":"abc", "text":"abc"}, {"id":"def", "text":"def"}]', '[{"id":"def"}]', 'Explanation for Question 7', 'Hint for Question 7', 2, 'Reason for difficulty 2');

-- Insert statement 8
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 8, 'hash8', 'Question 8 text', 'Question 8 description', '[{"id":"ghi", "text":"ghi"}, {"id":"jkl", "text":"jkl"}]', '[{"id":"jkl"}]', 'Explanation for Question 8', 'Hint for Question 8', 3, 'Reason for difficulty 3');

-- Insert statement 9
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 9, 'hash9', 'Question 9 text', 'Question 9 description', '[{"id":"mno", "text":"mno"}, {"id":"pqr", "text":"pqr"}]', '[{"id":"pqr"}]', 'Explanation for Question 9', 'Hint for Question 9', 4, 'Reason for difficulty 4');

-- Insert statement 10
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 10, 'hash10', 'Question 10 text', 'Question 10 description', '[{"id":"stu", "text":"stu"}, {"id":"vwx", "text":"vwx"}]', '[{"id":"vwx"}]', 'Explanation for Question 10', 'Hint for Question 10', 5, 'Reason for difficulty 5');

-- Insert statement 11
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 11, 'hash11', 'Question 11 text', 'Question 11 description', '[{"id":"yz1", "text":"yz1"}, {"id":"234", "text":"234"}]', '[{"id":"234"}]', 'Explanation for Question 11', 'Hint for Question 11', 1, 'Reason for difficulty 1');

-- Insert statement 12
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 12, 'hash12', 'Question 12 text', 'Question 12 description', '[{"id":"567", "text":"567"}, {"id":"890", "text":"890"}]', '[{"id":"567"}]', 'Explanation for Question 12', 'Hint for Question 12', 2, 'Reason for difficulty 2');

-- Insert statement 13
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 13, 'hash13', 'Question 13 text', 'Question 13 description', '[{"id":"abc", "text":"abc"}, {"id":"def", "text":"def"}]', '[{"id":"def"}]', 'Explanation for Question 13', 'Hint for Question 13', 3, 'Reason for difficulty 3');

-- Insert statement 14
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 14, 'hash14', 'Question 14 text', 'Question 14 description', '[{"id":"ghi", "text":"ghi"}, {"id":"jkl", "text":"jkl"}]', '[{"id":"jkl"}]', 'Explanation for Question 14', 'Hint for Question 14', 4, 'Reason for difficulty 4');

-- Insert statement 15
INSERT INTO questions (course_id, chapter_id, id_hash, que_text, que_description, choices, answers, ans_explanation, ans_hint, difficulty, diff_reason)
VALUES (1, 15, 'hash15', 'Question 15 text', 'Question 15 description', '[{"id":"mno", "text":"mno"}, {"id":"pqr", "text":"pqr"}]', '[{"id":"pqr"}]', 'Explanation for Question 15', 'Hint for Question 15', 5, 'Reason for difficulty 5');


-- Seed data for test_questions table
-- Test 1 questions
INSERT INTO test_questions (test_id, question_id) VALUES
      (1, 1), -- Test 1, Question 1
      (1, 2), -- Test 1, Question 2
      (1, 3), -- Test 1, Question 3
      (1, 4), -- Test 1, Question 4
      (1, 5), -- Test 1, Question 5
      (1, 6); -- Test 1, Question 6

-- Test 2 questions
INSERT INTO test_questions (test_id, question_id) VALUES
      (2, 2), -- Test 2, Question 2
      (2, 3), -- Test 2, Question 3
      (2, 4), -- Test 2, Question 4
      (2, 5), -- Test 2, Question 5
      (2, 6), -- Test 2, Question 6
      (2, 7); -- Test 2, Question 7

-- Test 3 questions
INSERT INTO test_questions (test_id, question_id) VALUES
      (3, 3), -- Test 3, Question 3
      (3, 4), -- Test 3, Question 4
      (3, 5), -- Test 3, Question 5
      (3, 6), -- Test 3, Question 6
      (3, 7), -- Test 3, Question 7
      (3, 1); -- Test 3, Question 1 (Repeated)

-- Test 4 questions
INSERT INTO test_questions (test_id, question_id) VALUES
      (4, 4), -- Test 4, Question 4
      (4, 5), -- Test 4, Question 5
      (4, 6), -- Test 4, Question 6
      (4, 7), -- Test 4, Question 7
      (4, 1), -- Test 4, Question 1 (Repeated)
      (4, 2); -- Test 4, Question 2 (Repeated)

-- Test 5 questions
INSERT INTO test_questions (test_id, question_id) VALUES
      (5, 5), -- Test 5, Question 5
      (5, 6), -- Test 5, Question 6
      (5, 7), -- Test 5, Question 7
      (5, 1), -- Test 5, Question 1 (Repeated)
      (5, 2), -- Test 5, Question 2 (Repeated)
      (5, 3); -- Test 5, Question 3 (Repeated)
