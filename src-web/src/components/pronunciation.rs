use wasm_bindgen::JsCast;
use web_sys::{HtmlAudioElement, HtmlButtonElement};
use yew::prelude::*;

pub struct Pronunciation {
    is_playing: bool,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub pinyin_numbers: String,
}

pub enum Msg {
    SetIsPlaying(bool),
}

impl Component for Pronunciation {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { is_playing: false }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let files = ctx
            .props()
            .pinyin_numbers
            .split(' ')
            .map(|py| format!("/public/audio/{py}.mp3"))
            .collect::<Vec<_>>();

        let files_data = files.join(",");

        let first_file = files.first().cloned();
        let onended = ctx.link().callback(move |ev: Event| {
            let mut audio: HtmlAudioElement = ev.target_unchecked_into();
            play_audio(&mut audio)
        });

        let onclick_play = ctx.link().callback(|ev: MouseEvent| {
            let (_, mut audio) = get_elements(ev);
            play_audio(&mut audio)
        });

        let onclick_stop = ctx.link().callback(|ev: MouseEvent| {
            let (_, mut audio) = get_elements(ev);
            stop_audio(&mut audio)
        });

        match first_file {
            Some(_) => html! {
            <div>
                <audio {onended} data-index={0} data-files={files_data} hidden={true}/>
                if self.is_playing {
                    <button variant={"secondary"} onclick={onclick_stop}>
                        <i class="las la-stop-circle" />
                        {"Stop"}
                    </button>
                } else {
                    <button variant={"secondary"} onclick={onclick_play}>
                        <i class="las la-play-circle" />
                        {"Play"}
                    </button>
                }
             </div>
            },
            None => html! {},
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetIsPlaying(is_playing) => self.is_playing = is_playing,
        }
        true
    }
}

fn play_audio(audio: &mut HtmlAudioElement) -> Msg {
    let index = audio
        .get_attribute("data-index")
        .expect("data-index should be set")
        .parse::<usize>()
        .expect("data-index should be parseable as usize");
    let files = get_files(audio);

    match files.get(index) {
        Some(file) => {
            audio.set_src(file);
            audio
                .set_attribute("data-index", &(index + 1).to_string())
                .unwrap();
            audio.play().ok();
            Msg::SetIsPlaying(true)
        }
        None => {
            // Out of index, reset
            stop_audio(audio);
            Msg::SetIsPlaying(false)
        }
    }
}

fn stop_audio(audio: &mut HtmlAudioElement) -> Msg {
    audio.set_attribute("data-index", "0").unwrap();
    let files = get_files(audio);
    audio.set_src(&files.first().cloned().unwrap_or_default());
    Msg::SetIsPlaying(false)
}

fn get_files(audio: &HtmlAudioElement) -> Vec<String> {
    let files_string = audio
        .get_attribute("data-files")
        .expect("data-files should be set");
    files_string.split(',').map(ToString::to_string).collect()
}

fn get_elements(ev: MouseEvent) -> (HtmlButtonElement, HtmlAudioElement) {
    let button: HtmlButtonElement = ev.target_unchecked_into();
    let audio: HtmlAudioElement = button
        .parent_element()
        .unwrap()
        .get_elements_by_tag_name("audio")
        .get_with_index(0)
        .unwrap()
        .unchecked_into();
    (button, audio)
}
