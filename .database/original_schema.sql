drop database if exists aarya_v1;
create database aarya_v1;
use aarya_v1;
create table courses (
    course_id int unsigned auto_increment primary key,
    course_name varchar(255) not null,
    course_description varchar(1024) not null
) auto_increment = 1000;
create table chapters (
    chapter_id int unsigned primary key,
    course_id int unsigned,
    chapter_name varchar(128),
    chapter_description varchar(512),
    constraint fk_chapters_courses foreign key (course_id) references courses (course_id)
) auto_increment = 1000;
create table topics (
    topic_id int unsigned auto_increment primary key,
    course_id int unsigned,
    chapter_id int unsigned,
    topic_name varchar(128),
    topic_description varchar(512),
    constraint fk_topics_courses foreign key (course_id) references courses (course_id),
    constraint fk_topics_chapters foreign key (chapter_id) references chapters (chapter_id)
) auto_increment = 1000;
create table questions (
    question_id int unsigned auto_increment primary key,
    course_id int unsigned not null,
    chapter_id int unsigned not null,
    topic_id int unsigned not null,
    que_text varchar(2048) not null,
    que_description varchar(2048) not null,
    choices varchar(2048) collate utf8mb4_bin not null check (json_valid(`choices`)),
    -- [{"id":"abc", "text":"abc"}, {"id":"def", "text":"def"}]
    answers varchar(2048) collate utf8mb4_bin not null check (json_valid(`answers`)),
    -- [{"id":"abc"}]
    ans_explanation varchar(2048) not null,
    ans_hint varchar(1024) not null,
    difficulty tinyint not null,
    diff_reason varchar(1024) not null,
    que_hash varchar(2048) null,
    constraint questions_q_hash_uindex unique (que_hash) using hash,
    constraint fk_questions_courses foreign key (course_id) references courses (course_id),
    constraint fk_questions_chapters foreign key (chapter_id) references chapters (chapter_id),
    constraint fk_questions_topics foreign key (topic_id) references topics (topic_id)
) auto_increment = 1000;
create index idx_question_course on questions (course_id);
create table students (
    student_id int unsigned auto_increment primary key,
    first_name varchar(255) not null,
    email_address varchar(255) not null,
    email_hash varchar(255) not null,
    pass_hash varchar(255) not null,
    over_13 bit not null,
    email_verified bit not null,
    account_active bit not null,
    constraint email_address unique (email_address)
) auto_increment = 10000;
create index idx_student_email on students (email_address);
create index students_email_address_hash_index on students (email_hash);
create table tests(
    test_id int unsigned auto_increment primary key,
    student_id int unsigned not null,
    course_id int unsigned not null,
    chapter_id int unsigned null,
    -- optional
    topic_id int unsigned null,
    -- optional
    test_difficulty tinyint not null,
    test_length tinyint not null,
    test_state tinyint not null,
    constraint fk_tests_courses foreign key (course_id) references courses (course_id),
    constraint fk_tests_students foreign key (student_id) references students (student_id)
) auto_increment = 1000;
create table test_questions (
    id int unsigned auto_increment primary key,
    test_id int unsigned not null,
    question_id int unsigned not null,
    -- 0: not attempted, 1: attempted, 2: correct, 3: incorrect
    question_state tinyint not null default 0,
    constraint fk_test_questions_tests foreign key (test_id) references tests (test_id),
    constraint fk_test_questions_questions foreign key (question_id) references questions (question_id)
) auto_increment = 1000;
create table students_history (
    history_id int unsigned auto_increment primary key,
    student_id int unsigned,
    action_type enum('update', 'delete', 'insert') not null check (action_type in ('update', 'delete', 'insert')),
    action_timestamp timestamp default current_timestamp(),
    first_name varchar(255) not null,
    email_address varchar(255) not null,
    email_hash varchar(255) not null,
    pass_hash varchar(255) not null,
    over_13 bit not null,
    email_verified bit not null,
    account_active bit not null
);
CREATE TRIGGER student_insert_trigger
AFTER
INSERT ON students FOR EACH ROW BEGIN
INSERT INTO students_history (
        student_id,
        action_type,
        first_name,
        email_address,
        email_hash,
        pass_hash,
        over_13,
        email_verified,
        account_active
    )
VALUES (
        new.student_id,
        'insert',
        new.first_name,
        new.email_address,
        new.email_hash,
        new.pass_hash,
        new.over_13,
        new.email_verified,
        new.account_active
    );
END;
CREATE TRIGGER student_update_trigger
AFTER
UPDATE ON students FOR EACH ROW BEGIN
INSERT INTO students_history (
        student_id,
        action_type,
        first_name,
        email_address,
        email_hash,
        pass_hash,
        over_13,
        email_verified,
        account_active
    )
VALUES (
        old.student_id,
        'update',
        old.first_name,
        old.email_address,
        old.email_hash,
        old.pass_hash,
        old.over_13,
        old.email_verified,
        old.account_active
    );
END;
CREATE TRIGGER student_delete_trigger
AFTER DELETE ON students FOR EACH ROW BEGIN
INSERT INTO students_history (
        student_id,
        action_type,
        first_name,
        email_address,
        email_hash,
        pass_hash,
        over_13,
        email_verified,
        account_active
    )
VALUES (
        old.student_id,
        'delete',
        old.first_name,
        old.email_address,
        old.email_hash,
        old.pass_hash,
        old.over_13,
        old.email_verified,
        old.account_active
    );
END;
insert into courses(course_name, course_description)
values(
        'AP Computer Science Principles',
        "AP Computer Science Principles is an elective course offered by the College Board's Advanced Placement (AP) program, providing high school students worldwide with an introductory college-level exploration of foundational computer science concepts. Through this course, students delve into computational thinking, problem-solving skills, and various aspects of computing, including programming, algorithms, data analysis, internet fundamentals, cybersecurity, and the societal impacts of technology. While typically not mandatory for graduation, students have the option to select this course to gain a deeper understanding of computer science principles and their applications."
    );
insert into courses(course_name, course_description)
values(
        'AP Computer Science A',
        "AP Computer Science A is an elective course offered by the College Board's Advanced Placement (AP) program, catering to high school students globally seeking an in-depth study of computer science fundamentals. Through this course, students delve into programming concepts using the Java programming language, focusing on topics such as algorithms, data structures, software development methodologies, and object-oriented programming principles. While not typically mandatory for graduation, students have the opportunity to choose this course to develop their programming skills and prepare for advanced study or careers in computer science and related fields."
    );
-- Insert statements for AP Computer Science Principles
INSERT INTO chapters (
        chapter_id,
        course_id,
        chapter_name,
        chapter_description
    )
VALUES (
        1000,
        1000,
        'The Internet',
        'Introduction to the Internet and basic web concepts'
    ),
    (
        1001,
        1000,
        'Digital Information',
        'Understanding digital information and representation'
    ),
    (
        1002,
        1000,
        'Algorithms and Programming',
        'Introduction to algorithms and basic programming concepts'
    ),
    (
        1003,
        1000,
        'Data',
        'Understanding data and data analysis'
    ),
    (
        1004,
        1000,
        'The Internet and Society',
        'Impact of the Internet on society and ethical considerations'
    );
-- Insert statements for AP Computer Science A
INSERT INTO chapters (
        chapter_id,
        course_id,
        chapter_name,
        chapter_description
    )
VALUES (
        1005,
        1001,
        'Primitive Data Types and Variables',
        'Introduction to primitive data types and variables in programming'
    ),
    (
        1006,
        1001,
        'Operators and Expressions',
        'Understanding operators and expressions in programming'
    ),
    (
        1007,
        1001,
        'Control Flow',
        'Introduction to control flow statements such as if, else, and loops'
    ),
    (
        1008,
        1001,
        'Arrays',
        'Understanding arrays and array manipulation in programming'
    ),
    (
        1009,
        1001,
        'Methods',
        'Introduction to methods/functions and modular programming concepts'
    );
-- Insert statements for AP Computer Science Principles topics
INSERT INTO topics (
        topic_id,
        course_id,
        chapter_id,
        topic_name,
        topic_description
    )
VALUES (
        1000,
        1000,
        1000,
        'Introduction to the Internet',
        'Basic concepts of the Internet and its importance in modern society'
    ),
    (
        1001,
        1000,
        1000,
        'History of the Internet',
        'Overview of the development and evolution of the Internet'
    ),
    (
        1002,
        1000,
        1000,
        'Internet Protocols',
        'Explanation of common Internet protocols such as HTTP, TCP/IP, and DNS'
    ),
    (
        1003,
        1000,
        1001,
        'Binary Representation',
        'Understanding binary representation of data and information'
    ),
    (
        1004,
        1000,
        1001,
        'Data Compression',
        'Explanation of data compression techniques and their applications'
    ),
    (
        1005,
        1000,
        1002,
        'Introduction to Algorithms',
        'Basic concepts of algorithms and their role in problem-solving'
    ),
    (
        1006,
        1000,
        1002,
        'Sequential and Parallel Algorithms',
        'Comparison between sequential and parallel algorithms'
    ),
    (
        1007,
        1000,
        1003,
        'Data Analysis Techniques',
        'Overview of data analysis techniques and tools'
    ),
    (
        1008,
        1000,
        1003,
        'Data Visualization',
        'Explanation of data visualization techniques for effective communication'
    ),
    (
        1009,
        1000,
        1004,
        'Cybersecurity',
        'Introduction to cybersecurity principles and best practices'
    ),
    (
        1010,
        1000,
        1004,
        'Privacy and Digital Citizenship',
        'Discussion on privacy issues and responsible digital citizenship'
    );
-- Insert student 1
INSERT INTO students (
        first_name,
        email_address,
        email_hash,
        pass_hash,
        over_13,
        email_verified,
        account_active
    )
VALUES (
        'John',
        'john@example.com',
        'hash1',
        'pass_hash1',
        1,
        1,
        1
    );
-- Insert student 2
INSERT INTO students (
        first_name,
        email_address,
        email_hash,
        pass_hash,
        over_13,
        email_verified,
        account_active
    )
VALUES (
        'Alice',
        'alice@example.com',
        'hash2',
        'pass_hash2',
        1,
        1,
        1
    );
-- Insert student 3
INSERT INTO students (
        first_name,
        email_address,
        email_hash,
        pass_hash,
        over_13,
        email_verified,
        account_active
    )
VALUES (
        'Bob',
        'bob@example.com',
        'hash3',
        'pass_hash3',
        1,
        0,
        0
    );