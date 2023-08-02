extern crate base64;
#[cfg(target_arch = "wasm32")]
use base64::encode;
#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;

extern crate web_sys;
#[cfg(target_arch = "wasm32")]
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};

#[cfg(target_arch = "wasm32")]
use gloo::file::callbacks::FileReader;
#[cfg(target_arch = "wasm32")]
use gloo::file::File;

#[cfg(target_arch = "wasm32")]
use yew::html::TargetCast;
#[cfg(target_arch = "wasm32")]
use yew::{html, Callback, Component, Context, Html};

#[cfg(target_arch = "wasm32")]
use pdfium_render::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use {
    once_cell::sync::Lazy,
    regex::Regex,
};

struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
    text: Vec<String>,
}

//passed through callbacks and handled by the update method
//used by components to communicate top-level changes
pub enum Msg {
    Loaded(String, String, Vec<u8>),
    Files(Vec<File>),
    Bubble(String),
}

//the top-level Yew component
pub struct App {
    readers: HashMap<String, FileReader>,
    files: Vec<FileDetails>,                                                                            //top-level file storage
    graph: (HashMap<(usize,usize),Vec<(usize,usize)>>,Vec<(String, (usize, usize), usize, usize)>),     //tuple of links and word coordinates, used by view_file method to create svg elements
    file_num : usize,                                                                                   //token used to generate graph after all files have been loaded
    word_vec: Vec<Vec<(String, Vec<String>)>>,
}

//required methods for Yew component lifecycle
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {readers: HashMap::default(), files: Vec::default(), graph: (HashMap::new(), Vec::new()), file_num: 0, word_vec: Vec::new(),}
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_name, file_type, data) => {
                let t = read_pdf(data.clone());
                self.files.push(FileDetails {
                    data: data.clone(), file_type, text : t, name: file_name.clone(), 
                });
                self.readers.remove(&file_name);

                if self.files.len() == self.file_num {
                    self.word_vec = self.files
                                        .iter()
                                        .map(|f| map_vec(generate_map(&f.text)))
                                        .collect::<Vec<_>>();

                    self.graph = draw(self.word_vec.clone());
                    web_sys::console::log_1(&"Made Graph".into());
                }

                true
            }
            Msg::Files(files) => {
                self.file_num += files.len();
                for file in files.into_iter() {
                    let file_name = file.name();
                    let file_type = file.raw_mime_type();

                    let task = {
                        let link  = ctx.link().clone();
                        let file_name = file_name.clone();

                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_name, file_type, res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
            Msg::Bubble(s) => {
                web_sys::console::log_1(&format!("{}", s).into());
                true
            }
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <div style="background-color: #18181a; position: absolute; top: 0; left: 0; width: 100%; height: 100%;">
                <div style="background-color: #9d9db0;">
                    <input 
                        type="file"
                        accept=".pdf"
                        multiple={true}
                        onchange={ctx.link().callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Self::upload_files(input.files())
                        })}
                    />
                </div>
                <svg width={"100%"} height={"95%"}>
                    {Self::view_file(&self, ctx)}
                </svg>
            </div>    
        }
    }
}

impl App {
    //dynamically generates svg elements to Yew VDOM
    fn view_file(&self, ctx: &Context<Self>) -> Html {


        let mut lines : Vec<((usize,usize),(usize,usize))> = Vec::new();

        for (my_cord, tar_list) in self.graph.0.iter() {
            for tar in tar_list {

                lines.push((*my_cord, *tar));
            }
        }

        let cords = self.graph.1.clone();

        html! {
            <svg width={"100%"} height={"100%"} viewBox={"0 0 500 236"}>
                <g>

                    {for lines.iter().map(|line_c|  html! {
                            <line x1={format!("{}", line_c.0.0)} y1={format!("{}.5", line_c.0.1)} x2={format!("{}", line_c.1.0)} y2={format!("{}.5", line_c.1.1)} stroke={"#323240"} stroke-width={"0.7"} />
                                })}
                    

                    {for cords.into_iter().map(|blob| 
                        
                        {
                            let real_word = blob.0.clone();

                            let mut word = real_word.clone();


                            if word.len() > 15 {
                                word = String::from(&word[0..15]);
                            }

                            let w_cords = blob.1;
                            let p_num = blob.2;
                            let w_rank = blob.3;

                            let width = word.len() + 15;
                            let x = w_cords.0 + width / 2;
                            let y = w_cords.1 + 6;
                    

                            let s  = self.word_vec[p_num][w_rank].1.clone().into_iter().collect::<Vec<String>>().join("\n");
                            
                            html! {
                                <>
                                    <rect x={format!("{}", w_cords.0)} y={format!("{}", w_cords.1)} width={format!("{}", width)} height={"10"} rx={"1.5"} fill={"#9d9db0"} onclick={ctx.link().callback(move |_| Msg::Bubble(format!("Appearances for {}: \n{}", real_word.clone(), s)))}/>
                                    <text x={format!("{}", x)} y={format!("{}.5", y)} fill={"#18181a"} font-family={"system-ui"} font-size={"4px"} text-anchor={"middle"} alignment-baseline={"middle"}>{word}</text> 
                                </>
                            }
                        }
                    )}
                </g>
            </svg>
        }
    }

    fn upload_files(files: Option<FileList>) -> Msg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);                
        }
        Msg::Files(result)
    }
}

//generates instance of pdfium to extract text from pdf
//takes ownership of buffer
fn read_pdf(buffer: Vec<u8>) -> Vec<String> {
    let mut pdf_vec = Vec::new();

    let bindings = Pdfium::bind_to_system_library().unwrap();
    Pdfium::new(bindings)
    .load_pdf_from_byte_vec(buffer, None).unwrap()
    .pages()
    .iter()
    .enumerate()
    .for_each(|(_, page)| {
        pdf_vec.push(page.objects()
            .iter()
            .filter_map(|object| object
                .as_text_object()
                .map(|object| object.text())) 
            .collect::<Vec<_>>()
            .join(" "))
    });

    pdf_vec
}

//lines and Coordinate vectors produced from top word vector
fn draw(vec: Vec<Vec<(String, Vec<String>)>>) -> (HashMap<(usize,usize),Vec<(usize,usize)>>,Vec<(String, (usize, usize), usize, usize)>) {
    
    let mut lines : HashMap<(usize,usize),Vec<(usize,usize)>> = HashMap::new();

    let mut cords: Vec<(String, (usize, usize), usize, usize)> = Vec::new();

    let center = (220, 110); 

    let r = 150;

    let doc_pos = surround(center, r, vec.len());

    for (i, doc) in vec.iter().enumerate() {

        let curr_center = doc_pos[i];
        let curr_radius = 35;

        let curr_pos = surround(curr_center, curr_radius, doc.len());

        for (j, word) in doc.iter().enumerate() {
            
            let my_pos = curr_pos[j];
            let my_word = word.0.clone();
            
            cords.push((my_word, my_pos, i, j));

        }
    }

    for l_word in cords.iter() {

        let my_word = &l_word.0;
        let my_pos = l_word.1;
        let my_page = l_word.2;
        let my_num = l_word.3;

        for target in cords.iter() {

            let tar_word = &target.0;
            let tar_pos = target.1;
            let tar_page = target.2;
            let tar_num = target.3;

            let mut line_exists_with_tar : bool = false;

            if lines.contains_key(&tar_pos) && lines[&tar_pos].contains(&my_pos) {
                line_exists_with_tar = true;
            }

            if tar_pos != my_pos && !line_exists_with_tar {

                for snippet in &vec[tar_page][tar_num].1 {
                    if snippet.contains(&my_word.clone()) {

                        let mut my_word_len = my_word.len();
                        let mut tar_word_len = tar_word.len();

                        if my_word_len > 15 {
                            my_word_len = 15;
                        }

                        if tar_word_len > 15 {
                            tar_word_len = 15;
                        }

                        let width1 = my_word_len + 6;
                        let x1 = my_pos.0 + width1 / 2;
                        let y1 = my_pos.1 + 6;

                        let width2 = tar_word_len + 6;
                        let x2 = tar_pos.0 + width2;
                        let y2 = tar_pos.1 + 6;

                        if lines.contains_key(&(x1, y1)) {
                            lines.entry((x1, y1))
                                .and_modify(|e| e.push((x2, y2)));
                        }

                        else {
                            lines.insert( (x1, y1), Vec::from([(x2, y2)]) );
                        }

                    }
                }
            }
        }
    }

    (lines, cords)
}

//helper function for mapping both individual doc positions and word positions
fn surround(c: (usize, usize), r: usize, max_len: usize) -> Vec<(usize, usize)> {

    let mut cords: Vec<(usize, usize)> = Vec::new();

    let mut center = c;

    enum Dirs {
        Left,
        Right,
        Up,
        Down,
    }

    let mut dir = Dirs::Right;

    let mut dist = r;

    let mut dir_changes = 1;

    while cords.len() < max_len {

        let curr = center;

        cords.push(curr);

        match dir {
            Dirs::Left => {
                    center.0 -= r;
                    dist -= r;
                    if dist == 0 {
                        dir = Dirs::Down;
                        dist = dir_changes * r;
                    }
                },
            Dirs::Right => { 
                    center.0 += r; 
                    dist -= r; 
                    if dist == 0 {
                        dir = Dirs::Up;
                        dist = dir_changes * r;
                    } 
                },
            Dirs::Up => {
                    if r > 100 {
                        center.1 += (r / 4) * 2;
                    }
                    else {
                        center.1 += (r / 3) * 2;
                    } 
                    dist -= r;
                    if dist == 0 {
                        dir = Dirs::Left;
                        dir_changes += 1;
                        dist = dir_changes * r;
                    }
                },
            Dirs::Down => {
                    if r > 100 {
                        center.1 -= (r / 4) * 2;
                    }
                    else {
                        center.1 -= (r / 3) * 2;
                    } 
                    dist -= r;
                    if dist == 0 {
                        dir = Dirs::Right;
                        dir_changes += 1;
                        dist = dir_changes * r;

                    }
            },
        }
    }

    cords
}


fn map_vec(m: HashMap<String, Vec<String>>) -> Vec<(String, Vec<String>)> {
    let mut vec : Vec<(String, Vec<String>)> = m.into_iter().collect::<Vec<(String, Vec<String>)>>();


    vec.sort_by(|a,b| b.1.len().cmp(&a.1.len()));

    let max = 10;
    
    if vec.len() > max {
        vec = vec[0..max+1].to_vec();
    }

    vec   
}

//parse text and create word: vec<appearance> pairs
fn generate_map(text: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut unique: HashMap<String, Vec<String>> = HashMap::new();
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-zA-z]+\S*[a-zA-z]").unwrap());

    for page in text {
        let page_vec = page.split(" ").collect::<Vec<_>>();

        for (i, s) in page_vec.iter().enumerate() {

            if let Some(w) = RE.find(s) {

                let word = w.as_str();

                if f_the(word) {

                    let mut iter = i-3..i+4;

                    if i < 3 && page_vec.len() <= i+3 {

                        iter = 0..page_vec.len();
                    }

                    else if i < 3 && page_vec.len() > i+3 { 
                        
                        iter = 0..i+3; }

                    else if page_vec.len() <= i+3 && i >= 3 { 
                        
                        iter = i-3..page_vec.len(); }

                    unique.entry(word.to_string())
                        .and_modify(|e| e.push(page_vec[iter.clone()].join(" ")))
                        .or_insert(vec![page_vec[iter.clone()].join(" ")]);
                }

            }
        }
    }

    unique
}

//filter common inconsequential words
//avoids polluting selected words for network graph
fn f_the(s: &str) -> bool {
    let x = String::from(
    "a an the all another any anybody anyone anything as both each either enough everybody everyone everything few he her 
    hers herself him himself his i it its itself many me mine most my myself neither nobody one other others our 
    ours ourself ourselves several she some somebody someone something somewhat such that their theirs them themselves 
    there these they this those us we what whatever where whereby wherein wherever whether which whichever who whoever 
    whom whomever whose you your yours yourself yourselves can could will would shall should may might must do does did 
    have has had get got go goes going be is am are was were been being very too only not no how as at by for from in of 
    per to because of for and nor or so yet as if why when with within without via until through than since onto on into 
    but then");
    
    if x.contains(&s.to_lowercase()) {
        return false;
    }
    true
}



#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn yew_app() {
    yew::Renderer::<App>::new()
    .render();
}

fn main() {
}
