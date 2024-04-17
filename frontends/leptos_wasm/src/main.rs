use gloo::console::log;
use gloo_net::http::Request;
use leptos::*;
use models::question_model::QuestionModel;

#[component]
fn App() -> impl IntoView {
    // load questions from the api

    let load_questions = create_resource(
        || (),
        |_| async move {
            // make the api call
            match Request::get("http://localhost:8080/questions").send().await {
                Ok(response) => {
                    let questions: Vec<QuestionModel> = response.json().await.unwrap();
                    Some(questions)
                }
                Err(e) => {
                    log!("Failed to load questions", e.to_string());
                    None
                }
            }
        },
    );

    view! { {
        move || match load_questions.get() {
            Some(qq) => {
                qq.unwrap().iter().map(|q| {
                    view! {
                        <div>
                            <h1>{&q.id_hash}</h1>
                            <p>{&q.que_text}</p>
                        </div>
                    }
                }).collect_view()
            },
            None => {
                view! {
                    <div>
                        <h1>{"Failed to load questions"}</h1>
                    </div>
                }.into_view()
            }
        }
    }}
}
fn main() {
    leptos::mount_to_body(App)
}
