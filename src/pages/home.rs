use leptos::*;  // Leptos для создания реактивных компонентов
use web_sys::console;  // для логирования в консоль браузера
use reqwasm::http::Request;  // для выполнения HTTP-запросов из wasm
use wasm_bindgen_futures::spawn_local;  // для запуска асинхронных операций в текущем потоке
use scraper::{Html, Selector};  // для парсинга HTML
use rand::seq::SliceRandom; use rand::thread_rng;  // для выбора случайного класса
use leptos::ev::SubmitEvent;

#[component]
pub fn Home() -> impl IntoView {
    // Создаем сигнал для хранения запроса, введенного пользователем | несмотря на ссылку в value=""(речь про <input type=text>), сигнал, связанный с этим полем нужно тоже инициализировать по умолчанию этой же ссылкой
    let (search_request, set_search_request) = create_signal("rust".to_string());  // p.s. иначе если сходу нажать на подгрузить, то компонент не тригернёт и value не будет взят для ссылки

    // Создаем сигнал для хранения количества изображений, которые нужно загрузить
    let (num_images, set_num_images) = create_signal(1);

    // Обработчик клика по кнопке "Подгрузить"
    let submit = {
        let search_request = search_request.clone(); // Клонируем сигнал search_request для использования в замыкании
        let num_images = num_images.clone(); // Клонируем сигнал num_images для использования в замыкании
        move |e: SubmitEvent| {
            e.prevent_default(); // Предотвращаем отправку формы

            let url_value = search_request.get(); // Получаем текущее значение URL
            let num_images_value = num_images.get(); // Получаем текущее значение количества изображений

            // Логируем значения в консоль для отладки
            console::log_1(&url_value.clone().into());
            console::log_1(&num_images_value.to_string().into());

            // Формируем URL для запроса с использованием введенного пользователем URL | CORS-ANYWHERE + UNSPLASH
            let url = format!("https://cors-anywhere.herokuapp.com/https://unsplash.com/s/photos/{}", url_value);
            console::log_1(&format!("Полученная ссылка для парсинга: {:?}", url).into());
            // Выполнение асинхронного GET-запроса
            spawn_local(async move {
                // Отправляем GET-запрос и ждем ответа
                match Request::get(&url).send().await {
                    // Если запрос выполнен успешно
                    Ok(response) => {
                        if response.ok() {  // Если ответ успешный (код 200)
                            let body = response.text().await.unwrap(); // Получаем текст ответа
                            parse_html_and_log_images(&body, num_images_value); // Парсим HTML и логируем изображения
                        } else {
                            console::log_1(&"запрос не 200(OK)".into()); // Логируем ошибку, если ответ не успешный
                        }
                    }
                    // Если произошла ошибка при выполнении запроса
                    Err(err) => {
                        console::log_1(&format!("Ошибка: {:?}", err).into()); // Логируем ошибку
                    }
                }
            });
        }
    };

    // Формируем HTML представление компонента
    view! {
        <form class="m-4 flex justify-between" on:submit=submit>  // форма для валидации(max и min), чтобы не было -10 картинок, а то там работает, но не совсем так как ожидается
            <a href="https://github.com/smir-ant/wasm_parser">
                <img class="inline-block w-12 mr-4 dark:invert animate-pulse" src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/91/Octicons-mark-github.svg/1200px-Octicons-mark-github.svg.png" />
            </a>
            // Поле ввода для URL
            <input class="mr-2 w-[65%]" type="text" value="rust" placeholder="что будем искать?" on:input=move |e| set_search_request(event_target_value(&e)) />
            // Поле ввода для количества изображений
            <input class="mr-2 w-[20%]" type="number" min="1" max="999" placeholder="кол-во изображений" on:input=move |e| set_num_images(event_target_value(&e).parse().unwrap_or(1)) />
            // Кнопка для отправки запроса
            <button>{"Подгрузить"}</button>
            </form>
        // Контейнер для изображений
        <div id="images-container"></div>
    }
}

// Функция для парсинга HTML и логирования изображений
fn parse_html_and_log_images(html: &str, num_images: usize) {
    // Парсинг HTML с помощью библиотеки scraper
    let document = Html::parse_document(html); // Создаем документ из HTML строки
    let selector = Selector::parse("img[src]").unwrap(); // Создаем селектор для выбора всех тегов img с атрибутом src

    let window = web_sys::window().expect("отсутствует a Window"); // Получаем объект окна браузера
    let web_document = window.document().expect("отсутствует Document"); // Получаем объект документа браузера
    let images_container = web_document.get_element_by_id("images-container").expect("отсутствует #images-container"); // Получаем контейнер для изображений
    images_container.set_inner_html("");  // Очистка содержимого контейнера | если несколько раз был запрос
    let mut will_show_count = 0;

    // Перебираем найденные элементы img и добавляем их на страницу
    for (i, element) in document.select(&selector).enumerate() {
        if will_show_count >= num_images {
            break; // Прерываем цикл, если добавлено достаточно изображений
        }

        if let Some(src) = element.value().attr("src") {
            if src.starts_with("data:") || src.starts_with("https://images.unsplash.com/profile") || src.starts_with("https://images.unsplash.com/placeholder"){ continue; }  // фильтруем лишнее
            will_show_count += 1;  // выше убрали мусор, и теперь вроде как норм фотка, поэтому +1
            console::log_1(&src.into()); // Логируем src изображения в консоль

            // Создание элемента img с помощью web_sys
            let img = web_document.create_element("img").unwrap(); // Создаем <img>
            img.set_attribute("src", src).unwrap(); // Устанавливаем атрибут src

            // делаем ссылку, в которую будет помещена картинка
            let link = web_document.create_element("a").unwrap(); // Создаем <a>
            link.set_attribute("href", src).unwrap(); // Устанавливаем атрибут href в <a>
            link.append_child(&img).unwrap();  // <img> идёт внутрь <a>
            link.set_attribute("class", ["size1", "size2", "size3", "size4"].choose(&mut thread_rng()).unwrap()).unwrap(); // случайный класс для размера

            images_container.append_child(&link).unwrap(); // вкладываем ссылку+картинку в контейнер
        }
    }
}