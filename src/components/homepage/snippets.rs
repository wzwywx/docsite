use syntect_html::syntect_html_fs;
use dioxus::prelude::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Snippet {
    pub title: &'static str,
    pub filename: &'static str,
    pub html: &'static str,
}

pub static SNIPPETS: &[Snippet] = &[
    Snippet {
        title: "Hello world",
        filename: "readme.rs",
        html: syntect_html_fs!("./snippets/readme.rs"),
    },
    Snippet {
        title: "Components",
        filename: "components.rs",
        html: syntect_html_fs!("./snippets/components.rs"),
    },
    Snippet {
        title: "Async",
        filename: "async.rs",
        html: syntect_html_fs!("./snippets/async.rs"),
    },
    Snippet {
        title: "Fetching",
        filename: "fetching.rs",
        html: syntect_html_fs!("./snippets/fetching.rs"),
    },
    Snippet {
        title: "Global State",
        filename: "fermi.rs",
        html: syntect_html_fs!("./snippets/fermi.rs"),
    },
];

pub fn Snippets(cx: Scope) -> Element {
    let selected_snippet = use_state(cx, || 0);

    cx.render(rsx! {
        section { class: "dark:text-white mt-4 -mx-4 sm:mx-0 lg:mt-0 lg:col-span-7 xl:col-span-6",
            div { class: "relative overflow-hidden min-h-0 flex-auto flex-col flex bg-ghmetal max-h-[60vh] sm:max-h-[none] sm:rounded-xl dark:backdrop-blur border border-neutral-500/30 shadow-cutesy",
                div { class: "flex-none overflow-auto whitespace-nowrap flex relative min-w-full bg-ghdarkmetal pt-3 px-3",
                    ul { class: "flex text-sm leading-6 text-gray-100",
                        SNIPPETS.iter().enumerate().map(|(id, snippet)| {
                            let selected = **selected_snippet == id;

                            let bg_selected = match selected {
                                true => "bg-ghmetal border-neutral-500/30 border text-white border-b-0",
                                false => "bg-ghdarkmetal",
                            };

                            rsx! {
                                li { class: "flex-none",
                                    button { class: "relative py-2 px-4 rounded-t-md {bg_selected}", r#type: "button", onclick: move |_| selected_snippet.set(id),
                                        "{snippet.filename}"
                                        if selected {
                                            Some(rsx!{ span { class: "absolute z-10 bottom-0 inset-x-0 h-2 bg-ghmetal" } })
                                            // Some(rsx!{ span { class: "absolute z-10 bottom-0 inset-x-3 h-px bg-ghmetal" } })
                                        } else {
                                            None
                                        }
                                    }
                                }
                            }
                        })
                    }
                    div { class: "absolute bottom-0 inset-x-0 h-px bg-neutral-500/30" }
                }

                div {
                    SNIPPETS.iter().enumerate().map(|(id, snippet)| {
                        // Instead of hiding/showing, we just render all the code blocks at once and hide them with css instead
                        let show = match **selected_snippet {
                            a if a == id => "block",
                            _ => "hidden"
                        };

                        rsx! {
                            div {
                                key: "{snippet.title}",
                                class: "w-full min-h-0 p-4 {show}",
                                background_color: "#2b303b",
                                dangerous_inner_html: "{snippet.html}"
                            }
                        }
                    })
                }
            }
        }
    })
}

// div { class: "relative overflow-hidden flex bg-neutral-800 max-h-[60vh] sm:max-h-[none] sm:rounded-xl dark:bg-neutral-900/70 dark:backdrop-blur dark:ring-1 dark:ring-inset dark:ring-white/10 shadow-3xl",
// div { class: "relative overflow-hidden flex bg-neutral-800 h-[31.625rem] max-h-[60vh] sm:max-h-[none] sm:rounded-xl lg:h-[34.6875rem] xl:h-[31.625rem] dark:bg-neutral-900/70 dark:backdrop-blur dark:ring-1 dark:ring-inset dark:ring-white/10 shadow-3xl",
// div { class: "relative w-full flex flex-col",
// div { class: "flex-none",
//     div { class: "flex items-center h-8 space-x-1.5 px-3",
//         div { class: "w-2.5 h-2.5 bg-red-600 rounded-full" }
//         div { class: "w-2.5 h-2.5 bg-yellow-600 rounded-full" }
//         div { class: "w-2.5 h-2.5 bg-green-600 rounded-full" }
//     }
// }
