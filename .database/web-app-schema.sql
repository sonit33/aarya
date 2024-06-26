drop database if exists aarya_v1;
create database aarya_v1;
use aarya_v1;
create table courses (
    course_id int unsigned primary key,
    course_name varchar(255) not null,
    course_description varchar(1024) not null,
    course_name_hash varchar(2048) null,
    constraint unique_course_name_hash unique (course_name_hash)
);
create table chapters (
    chapter_id int unsigned primary key,
    course_id int unsigned,
    chapter_name varchar(128),
    chapter_description varchar(512),
    chapter_name_hash varchar(2048) null,
    constraint unique_chapter_name_hash unique (chapter_name_hash),
    constraint fk_chapters_courses foreign key (course_id) references courses (course_id)
);
create table topics (
    topic_id int unsigned primary key,
    course_id int unsigned,
    chapter_id int unsigned,
    topic_name varchar(128),
    topic_description varchar(512),
    constraint fk_topics_courses foreign key (course_id) references courses (course_id),
    constraint fk_topics_chapters foreign key (chapter_id) references chapters (chapter_id)
);
create table questions (
    question_id int unsigned primary key,
    course_id int unsigned not null,
    chapter_id int unsigned not null,
    topic_id int unsigned not null,
    que_text varchar(2048) not null,
    que_description varchar(2048) not null,
    choices varchar(2048) collate utf8mb4_bin not null check (json_valid(`choices`)),
    -- [{"id":"abc", "text":"abc"}, {"id":"def", "text":"def"}]
    answers varchar(2048) collate utf8mb4_bin not null check (json_valid(`answers`)),
    -- [{"id":"abc"}]
    radio bit null default 0,
    ans_explanation varchar(2048) not null,
    ans_hint varchar(1024) not null,
    difficulty tinyint not null,
    diff_reason varchar(1024) not null,
    que_hash varchar(2048) null,
    constraint unique_questions_q_hash unique (que_hash) using hash,
    constraint fk_questions_courses foreign key (course_id) references courses (course_id),
    constraint fk_questions_chapters foreign key (chapter_id) references chapters (chapter_id),
    constraint fk_questions_topics foreign key (topic_id) references topics (topic_id)
);
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
    constraint unique_email_address unique (email_address)
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

insert into students(first_name, email_address, email_hash, pass_hash, over_13, email_verified, account_active) values("Jon", "jon@abc.com","not-set","not-set",1,1,1);