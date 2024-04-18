use gloo::console::log;
use gloo_net::{self, http::Request};
use leptos::*;
use models::CourseQueryModel;

#[component()]
pub fn AllCourses() -> impl IntoView {
    let api = create_resource(
        || (),
        |_| async move {
            match Request::get("http://localhost:9090/courses").send().await {
                Ok(response) => {
                    if response.ok() {
                        let data = response.text().await.unwrap();
                        Some(data)
                    } else {
                        log!("Error", response.text().await.unwrap());
                        None
                    }
                }
                Err(e) => {
                    log!("Error", e.to_string());
                    None
                }
            }
        },
    );

    view! {
        <>
            {
                move || match api.get() {
                    Some(data) => {
                        let courses: Vec<CourseQueryModel> = serde_json::from_str(&data.unwrap()).unwrap();
                        view! {
                            {courses.iter().map(|course| {
                                    let link = format!("/course/{}/chapters", &course.id_hash);
                                    view! {
                                        <div>
                                            <a href={link}>{&course.name}</a>
                                        </div>
                                    }.into_view()
                                }).collect_view()}
                        }.into_view()
                    },
                    None => view!{"Loading..."}.into_view(),
                }
            }
        </>
    }
}
