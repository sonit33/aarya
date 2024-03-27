**MODELS**

- students
- teachers
- courses (start with AP CS-P and CS-A)
- questions
- tests
- practices
- consultations
- tutoring
- assignments

**Entity Relationship**

- student
  - pk: student_id
  - first_name
  - email_address
  - password
  - over_13
  - email_verified
  - account_active
  - added_timestamp
  - updated_timestamp
  - deleted_timestamp
- courses
  - pk: course_id
  - course_name
  - added_timestamp
  - updated_timestamp
  - description
- topics
  - pk: topic_id
  - fk: course_id (a topic belongs to only one course)
  - topic_name
  - description
  - added_timestamp
  - updated_timestamp
- teachers
  - pk: teacher_id
  - teacher_name
  - teacher_school
  - password
  - email_address
  - photo_url
  - blurb
  - education
  - skills
  - certifications
  - employed_at
  - availability (weekends, every-day, weekdays, morning, afternoon, evening, all-day, custom)
  - account_active
  - venmo_handle
  - paypal_handle
  - added_timestamp
  - updated_timestamp
  - deleted_timestamp
  - feedbacks
- questions
  - pk: question_id
  - fk: course_id (a question can belong to only one course)
  - fk: topic_id (a question may have one or more topics)
  - question
  - answers (text if free-text, one or more otherwise)
  - choices (min: 4; max: 6; 0 if free-text)
  - q_difficulty (on a scale from 1 to 5)
  - d_reason
  - a_explanation
  - a_hint
  - q_mode (1=multiple-choice, 2=single-choice, 3=free-test)
  - feedbacks
- tests
  - pk: test_id
  - fk: course_id (a test will belong to only one course)
  - fk: question_id (a test will have one or more questions)
  - fk: student_id (a student can take one or more tests)
  - fk: topic_id (a test may have zero or more topics; zero if its a test because all topics will be covered; one or
    more if its a practice)
  - test_name
  - feedbacks
  - added_timestamp
  - updated_timestamp
  - score
  - result (graded e.g. A, A-, F)
  - score_sheet (json: {question_id:1, answers:[aaa,bbbb], correct_answers: [ccc, aaa], result: [0,1]})
  - aarya_recommendations
  - mode (1=test or 2=practice)
  - test_size (1=small, 2=medium, 3=large)
- payments
  - pk: payment_id
  - p_direction (student to Aarya, Aarya to teacher)
  - p_amount
  - added_timestamp
  - p_id (points to payment_id if a new entry reverses or changes the original entry)
  - student_id
  - teacher_id
  - purpose (tutoring or commission for onboarded teachers)
- tutoring
  - pk: tutoring_id
  - fk: course_id (each tutoring will have only one course)
  - fk: student_id (a teacher can tutor one or more students)
  - fk: teacher_id (each tutoring will have one teacher)
  - t_type (one-time consultation, trial, package e.g. 10-hours, long-term e.g. monthly billing)
- assignments
  - pk: assignment_id
  - fk: course_id (an assignment may belong to one only course)
  - fk: teacher_id (a teacher can create multiple assignments)
  - fk: student_id (an assignment may be given to one or more students)
  - fk: topic_id (an assignment may have one or more topics)
  - assignment_name
  - added_timestamp
  - due_on_timestamp

**PAGES**

**common**

- index
- auth
  - signup
  - login
  - forgot-password
  - reset-password
- blogs

**student web view**

- profile
  - tests
  - practices
  - account
- tests
  - start a new test
- practice
  - start a new practice session
- consultations
  - start a new consultation
    - calendar block
  - checkout
  - payment
- subscriptions
  - start a new subscription --> log-term consultation (1-month or more, cancel any time)

**teacher web view**

- dashboard
  - consultations
  - tests
  - availability
    - block or unblock calendar
- settings
  - hourly fee
- tests
  - develop a new custom test
- teaching material
  - add a new teaching material
- assignments
  - create a new long-form assignment
  - grading and scoring of long-form assignments

**admin cli view**

**WEBSITE WORKFLOWS**

- signup -> verify email -> login ✅
- login -> dashboard ✅
- forgot-password -> email address -> verification code -> reset password
- reset-password -> email address -> verification code -> reset password
- practice session -> new test form -> start session -> practice -> score
- test -> new test form -> start test -> score
- consultation (1:1) -> start new consultation form -> find a teacher form -> schedule (see availability and block
  calendar) -> payment -> appointment confirmed -> send email with meeting link
- subscription (1:1 10-hours or more) -> start new subscription -> find a teacher form -> schedule -> -> summary page ->
  payment -> email confirmation with meeting link
- courses (1:M) -> list running courses -> enroll in a course -> confirm schedule -> payment -> send email with meeting
  link

**Technical debt**

- introduce [crossbeam](https://github.com/crossbeam-rs/crossbeam) queues to send emails and other time-consuming tasks
