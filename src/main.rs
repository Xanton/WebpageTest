use leptos::*;

mod components;

use components::{header::Header, footer::Footer, main_content::MainContent};

fn main() {
    leptos::start(|cx| {
            leptos::view! { cx,
                        <div>
                                        <Header/>
                                                        <MainContent/>
                                                                        <Footer/>
                                                                                    </div>
                                                                                            }
                                                                                                });
                                                                                                }