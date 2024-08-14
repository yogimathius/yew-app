use gloo_net::http::Request;
use log::error;
use yew::prelude::*;
use yew_app::video::Video; // replace with your own path
use yew_app::video_details::VideoDetails;
use yew_app::videos_list::VideosList; // Ensure you have a logger set up

async fn fetch_videos() -> Result<Vec<Video>, Box<dyn std::error::Error>> {
    match Request::get("/tutorial/data.json").send().await {
        Ok(response) => match response.json().await {
            Ok(videos) => Ok(videos),
            Err(e) => {
                error!("Failed to parse JSON: {:?}", e);
                Err(Box::new(e))
            }
        },
        Err(e) => {
            error!("Failed to send request: {:?}", e);
            Err(Box::new(e))
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = match fetch_videos().await {
                    Ok(videos) => videos,
                    Err(e) => {
                        error!("Failed to fetch videos: {:?}", e);
                        vec![]
                    }
                };
                videos.set(fetched_videos);
            });
            || ()
        });
    }

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
