use leptos::*;

#[component]
pub fn Header(cx: Scope) -> Html {
    view! { cx,
            <header class="header">
                        <div class="container">
                                        <img src="/path/to/logo.png" alt="Company Logo" class="logo"/>
                                                        <nav>
                                                                            <ul>
                                                                                                    <li><a href="#branches">Branches</a></li>
                                                                                                                            <li><a href="#services">Services</a></li>
                                                                                                                                                    <li><a href="#references">References</a></li>
                                                                                                                                                                            <li><a href="#news">News</a></li>
                                                                                                                                                                                                    <li><a href="#about-us">About Us</a></li>
                                                                                                                                                                                                                            <li><a href="#contacts">Contacts</a></li>
                                                                                                                                                                                                                                                </ul>
                                                                                                                                                                                                                                                                </nav>
                                                                                                                                                                                                                                                                                <button class="language-switcher">Switch Language</button>
                                                                                                                                                                                                                                                                                            </div>
                                                                                                                                                                                                                                                                                                    </header>
                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                        }