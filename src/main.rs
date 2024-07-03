use leptos::*;
use leptos_meta::{provide_meta_context, Html, Meta, Title};
use leptos_router::{Router, Route, Routes};

mod pages;
use crate::pages::home::Home;


fn main() {
    mount_to_body(|| {
        provide_meta_context();  // контекст, который управляет различными аспектами веб-страницы, такими как стили, заголовки, мета-теги и т.д.
        // “контекст” обычно относится к набору данных или состоянию, которое доступно всему приложению или определенной его части

        view! {
            <Html lang="en" dir="ltr" />  // ltr (слева-направо направление текста)
            <Title text="WASM parser by smir-ant"/>  // название страницы
            <Meta charset="UTF-8"/>
            <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

            <Router>
                <Routes>
                    <Route path="/*" view=Home />
                </Routes>
            </Router>
        }
    })
}
