use leptos::*;
use leptos_router::*;

use crate::components::{chapters::AllChaptersByCourse, courses::AllCourses};

mod components;

#[component()]
fn App() -> impl IntoView {
    view! {
        <div id="main">
            <Router>
                <nav>
                    <ul>
                        <li><A href="/">"Home"</A></li>
                        <li><A href="/courses">"Available courses"</A></li>
                        <li><A href="/course/1/chapters">"Available chapters in course 1"</A></li>
                        <li><A href="/course/1/tests">"Available tests in course 1"</A></li>
                        <li><A href="/course/1/chapter/1/tests">"Available tests in chapter 1 in course 1"</A></li>
                        <li><A href="/tests">"All tests categorized by courses and chapters"</A></li>
                        <li><A href="/test/build">"Build a new test"</A></li>
                    </ul>
                </nav>
                <main>
                    <Routes>
                        /// home
                        <Route path="/" view= move || view!{"Home"}/>
                        /// get all courses
                        <Route path="/courses" view= move || AllCourses/>
                        /// get all chapters in a course
                        <Route path="/course/:id_hash/chapters" view= move || AllChaptersByCourse/>
                        /// get all tests in a course
                        <Route path="/course/1/tests" view= move || view!{"All tests in Course 1"}/>
                        /// get all tests in a chapter in a course
                        <Route path="/course/1/chapter/1/tests" view= move || view!{"All tests in Chapter 1, Course 1"}/>
                        /// get a test by id-hash
                        <Route path="/test/1" view= move || view!{"You picked test #1"}/>
                        /// not found
                        <Route path="/*any" view= move || view! { <h1>"Not Found"</h1> }/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
