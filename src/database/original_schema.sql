create table courses
(
    course_id         int auto_increment
        primary key,
    course_name       varchar(255)                          not null,
    added_timestamp   timestamp default current_timestamp() null,
    updated_timestamp timestamp default current_timestamp() null on update current_timestamp(),
    description       text                                  null
);

create index idx_course_name
    on courses (course_name);

create table feedbacks
(
    feedback_id int auto_increment
        primary key,
    course_id   int null,
    test_id     int null,
    question_id int null,
    teacher_id  int null,
    student_id  int null
);

create table questions
(
    question_id   int auto_increment
        primary key,
    course_id     int               not null,
    question      text              not null,
    answers       text              not null,
    choices       int               null
        check (`choices` between 0 and 6),
    q_difficulty  int               null
        check (`q_difficulty` between 1 and 5),
    d_reason      text              null,
    a_explanation text              null,
    a_hint        text              null,
    q_mode        tinyint default 1 not null,
    constraint questions_ibfk_1
        foreign key (course_id) references courses (course_id)
);

create index idx_question_course
    on questions (course_id);

create table students
(
    student_id        int auto_increment
        primary key,
    first_name        varchar(255)                          not null,
    email_address     varchar(255)                          not null,
    password          varchar(255)                          not null,
    over_13           tinyint(1)                            not null,
    email_verified    tinyint(1)                            not null,
    account_active    tinyint(1)                            not null,
    added_timestamp   timestamp default current_timestamp() null,
    updated_timestamp timestamp default current_timestamp() null on update current_timestamp(),
    deleted_timestamp timestamp                             null,
    constraint email_address
        unique (email_address)
);

create index idx_student_email
    on students (email_address);

create table teachers
(
    teacher_id        int auto_increment
        primary key,
    teacher_name      varchar(255)                          not null,
    teacher_school    varchar(255)                          null,
    password          varchar(255)                          not null,
    email_address     varchar(255)                          not null,
    photo_url         varchar(255)                          null,
    blurb             text                                  null,
    education         text                                  null,
    skills            text                                  null,
    certifications    text                                  null,
    employed_at       varchar(255)                          null,
    availability_dow  tinyint   default 1                   not null comment 'weekends, daily, weekdays, custom',
    account_active    tinyint(1)                            not null,
    venmo_handle      varchar(255)                          null,
    paypal_handle     varchar(255)                          null,
    added_timestamp   timestamp default current_timestamp() null,
    updated_timestamp timestamp default current_timestamp() null on update current_timestamp(),
    deleted_timestamp timestamp                             null,
    availability_tod  tinyint   default 1                   null comment 'morning, afternoon, evening, all-day, custom',
    constraint email_address
        unique (email_address)
);

create table assignments
(
    assignment_id    int auto_increment
        primary key,
    course_id        int                                   not null,
    teacher_id       int                                   not null,
    assignment_name  varchar(255)                          not null,
    added_timestamp  timestamp default current_timestamp() null,
    due_on_timestamp timestamp                             not null,
    constraint assignments_ibfk_1
        foreign key (course_id) references courses (course_id),
    constraint assignments_ibfk_2
        foreign key (teacher_id) references teachers (teacher_id)
);

create table assignment_students
(
    assignment_id int not null,
    student_id    int not null,
    constraint assignment_students_ibfk_1
        foreign key (assignment_id) references assignments (assignment_id),
    constraint assignment_students_ibfk_2
        foreign key (student_id) references students (student_id)
);

create index assignment_id
    on assignment_students (assignment_id);

create index student_id
    on assignment_students (student_id);

create index idx_assignment_course_teacher
    on assignments (course_id, teacher_id);

create index teacher_id
    on assignments (teacher_id);

create table payments
(
    payment_id      int auto_increment
        primary key,
    p_direction     tinyint   default 1                   not null,
    p_amount        double                                not null,
    added_timestamp timestamp default current_timestamp() null,
    p_id            int                                   null,
    student_id      int                                   null,
    teacher_id      int                                   null,
    purpose         tinyint   default 1                   not null,
    constraint payments_ibfk_1
        foreign key (student_id) references students (student_id),
    constraint payments_ibfk_2
        foreign key (teacher_id) references teachers (teacher_id)
);

create index student_id
    on payments (student_id);

create index teacher_id
    on payments (teacher_id);

create index idx_teacher_email
    on teachers (email_address);

create table tests
(
    test_id           int auto_increment
        primary key,
    course_id         int                                   not null,
    test_name         varchar(255)                          not null,
    added_timestamp   timestamp default current_timestamp() null,
    updated_timestamp timestamp default current_timestamp() null on update current_timestamp(),
    mode              tinyint   default 1                   not null,
    test_size         tinyint   default 1                   not null,
    constraint tests_ibfk_1
        foreign key (course_id) references courses (course_id)
);

create table test_questions
(
    test_id     int not null,
    question_id int not null,
    constraint test_questions_ibfk_1
        foreign key (test_id) references tests (test_id),
    constraint test_questions_ibfk_2
        foreign key (question_id) references questions (question_id)
);

create index question_id
    on test_questions (question_id);

create index test_id
    on test_questions (test_id);

create table test_students
(
    test_id     int           not null,
    student_id  int           not null,
    ai_feedback varchar(1024) null comment 'Aarya''s feedback about the test',
    constraint test_students_ibfk_1
        foreign key (test_id) references tests (test_id),
    constraint test_students_ibfk_2
        foreign key (student_id) references students (student_id)
);

create index student_id
    on test_students (student_id);

create index test_id
    on test_students (test_id);

create index idx_test_course
    on tests (course_id);

create table topics
(
    topic_id          int auto_increment
        primary key,
    course_id         int                                   not null,
    topic_name        varchar(255)                          not null,
    description       text                                  null,
    added_timestamp   timestamp default current_timestamp() null,
    updated_timestamp timestamp default current_timestamp() null on update current_timestamp(),
    constraint topics_ibfk_1
        foreign key (course_id) references courses (course_id)
);

create table assignment_topics
(
    assignment_id int not null,
    topic_id      int not null,
    constraint assignment_topics_ibfk_1
        foreign key (assignment_id) references assignments (assignment_id),
    constraint assignment_topics_ibfk_2
        foreign key (topic_id) references topics (topic_id)
);

create index assignment_id
    on assignment_topics (assignment_id);

create index topic_id
    on assignment_topics (topic_id);

create table question_topics
(
    question_id int not null,
    topic_id    int not null,
    constraint question_topics_ibfk_1
        foreign key (question_id) references questions (question_id),
    constraint question_topics_ibfk_2
        foreign key (topic_id) references topics (topic_id)
);

create index question_id
    on question_topics (question_id);

create index topic_id
    on question_topics (topic_id);

create table test_topics
(
    test_id  int not null,
    topic_id int not null,
    constraint test_topics_ibfk_1
        foreign key (test_id) references tests (test_id),
    constraint test_topics_ibfk_2
        foreign key (topic_id) references topics (topic_id)
);

create index test_id
    on test_topics (test_id);

create index topic_id
    on test_topics (topic_id);

create index course_id
    on topics (course_id);

create table tutoring
(
    tutoring_id int auto_increment
        primary key,
    course_id   int     not null,
    student_id  int     not null,
    teacher_id  int     not null,
    t_type      tinyint not null comment 'one-time consultation, trial, package, long-term',
    constraint tutoring_ibfk_1
        foreign key (course_id) references courses (course_id),
    constraint tutoring_ibfk_2
        foreign key (student_id) references students (student_id),
    constraint tutoring_ibfk_3
        foreign key (teacher_id) references teachers (teacher_id)
);

create index idx_tutoring_course_student_teacher
    on tutoring (course_id, student_id, teacher_id);

create index student_id
    on tutoring (student_id);

create index teacher_id
    on tutoring (teacher_id);

