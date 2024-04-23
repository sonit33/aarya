-- select * from students;
-- select *
-- from tests;
-- select *
-- from test_questions;
SELECT q.question_id,
    q.course_id,
    q.chapter_id,
    q.topic_id,
    q.que_text,
    q.que_description,
    q.choices,
    q.difficulty,
    q.diff_reason,
    q.ans_explanation,
    q.ans_hint,
    t.topic_name,
    c.course_name,
    ch.chapter_name
FROM questions q
    JOIN courses c ON q.course_id = c.course_id
    JOIN chapters ch ON q.chapter_id = ch.chapter_id
    JOIN topics t ON t.topic_id = q.topic_id
WHERE q.difficulty = 2
    and q.course_id = 1000
    and q.chapter_id = 0
    and q.topic_id = 0
LIMIT 25;