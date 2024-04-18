use core::str;

use gloo::console::log;
use gloo_net::http::Request;
use leptos::*;
use leptos_router::use_params_map;
use models::ChapterQueryModel;

#[component()]
pub fn AllChaptersByCourse() -> impl IntoView {
    let params = use_params_map();
    let id_hash = move || params.with(|params| params.get("id_hash").clone().unwrap());
    // let id_hash = id_hash().as_str();
    let api = create_resource(
        || (),
        |_| async move {
            match Request::get(format!("http://localhost:9090/chapters/course/{id_hash}").as_str()).send().await {
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
                        let model: Vec<ChapterQueryModel> = serde_json::from_str(&data.unwrap()).unwrap();
                        view! {
                            {model.iter().map(|chapter| {
                                    let link = format!("/tests/course/{:?}/chapter/{:?}", &chapter.course_id_hash, &chapter.id_hash);
                                    log!("Link", &link);
                                    view! {
                                        <div>
                                            <a href={link}>{&chapter.name}</a>
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
