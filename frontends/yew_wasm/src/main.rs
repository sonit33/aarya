use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(PartialEq, Properties)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    html! {
        <div>
            <h3>{"Videos to watch"}</h3>
            {
                videos.iter().map(|video| {
                    let on_video_select = {
                            let on_click = on_click.clone();
                            let video = video.clone();
                            Callback::from(move |_| {
                                on_click.emit(video.clone());
                            })
                        };

                        html! {
                            <p style="cursor: pointer;" key={video.id} onclick={on_video_select}>
                                {format!("{} by {}", video.title, video.speaker)}
                            </p>
                        }
                }).collect::<Html>()
            }
        </div>
    }
}

fn fetch_videos() {}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{video.title.clone()}</h3>
            <p>{video.speaker.clone()}</p>
            <a href={video.url.clone()}>{"Watch here"}</a>
        </div>
    }
}

#[function_component(Videos)]
fn videos() -> Html {
    let selected_video = use_state(|| None);
    let videos = use_state(|| vec![]);
    let videos = videos.clone();
    {
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("https://yew.rs/tutorial/data.json").send().await.unwrap().json().await.unwrap();
                videos.set(fetched_videos);
            });
            || ()
        });
    }
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video));
        })
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()}/>
        }
    });

    html! {
        <div>
            <VideoList videos={(*videos).clone()} on_click={on_video_select.clone()}/>
            {for details}
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <Videos/>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
