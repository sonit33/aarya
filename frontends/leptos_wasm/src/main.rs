use leptos::*;
use leptos_router::*;

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
                        /// get all tests
                        <Route path="/tests" view= move || view!{
                            <p>"Pick a test"</p>
                            <A href="/test/1">"Test #1"</A>
                        }/>
                        /// get a test by id-hash
                        <Route path="/test/1" view= move || view!{"You picked test #1"}/>
                        /// get all courses
                        <Route path="/courses" view= move || view!{"All courses"}/>
                        /// get all chapters in a course
                        <Route path="/course/1/chapters" view= move || view!{"All chapters in Course 1"}/>
                        /// get all tests in a course
                        <Route path="/course/1/tests" view= move || view!{"All tests in Course 1"}/>
                        /// get all tests in a chapter in a course
                        <Route path="/course/1/chapter/1/tests" view= move || view!{"All tests in Chapter 1, Course 1"}/>
                        /// build a new test
                        <Route path="/test/build" view= move || view!{"Start building a new test"}/>
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
