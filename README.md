# Парсер изображений на rust из-под frontend

###### Проект реализован 03.07.2024

<picture><img src="https://img.shields.io/badge/WASM(Leptos)-654FF0?style=for-the-badge&logo=rust&logoColor=white"></picture>
<picture><img src="https://img.shields.io/badge/reqwasm-654FF0?style=for-the-badge&logo=rust&logoColor=white"></picture>
<picture><img src="https://img.shields.io/badge/Trunk-f97316?style=for-the-badge&logo=rust&logoColor=white"></picture>
<picture><img src="https://img.shields.io/badge/scraper-f97316?style=for-the-badge&logo=rust&logoColor=white"></picture>
<picture><img src="https://img.shields.io/badge/rand-f97316?style=for-the-badge&logo=rust&logoColor=white"></picture>
<picture><img src="https://img.shields.io/badge/Tailwind-06B6D4?style=for-the-badge&logo=tailwindcss&logoColor=white"></picture>

<picture><img width="1000" src="https://github.com/smir-ant/smir-ant/assets/84059957/4039a4a0-baba-4335-afd5-93348b909f65"></picture>

<br>

## Моменты:
1) в поле для ввода принимается поисковый запрос, поиск осуществляется на фотостоке unsplash.Остальные<sub>(среди нормально-популярных с нестрёмными фотками)</sub> сервисы выдают либо 404, либо 403.
2) механизм мозайки примитивнейший: рандомно выбирается один из 4 классов(1x1, 2x1, 1x2 или 2x2) и кладётся в dom. так что нередко вы будете получать отверстия, можно починить если прикрутить [Masonry](https://masonry.desandro.com/), например, однако, честно говоря, западло

<details>
<summary>
типичный пример выдачи с отверстиями

</summary>
<picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://user-images.githubusercontent.com/84059957/215088292-cf50a16b-422b-43cc-a211-c4169553ca62.png">
    <source media="(prefers-color-scheme: light)" srcset="https://user-images.githubusercontent.com/84059957/210322548-b635bad5-c53d-4209-a73e-fb0adcc437bf.png">
    <img height="0.8">
</picture>

<picture><img width="1000" src="https://github.com/smir-ant/smir-ant/assets/84059957/50efd0ae-0e00-4ae9-b953-be4455f63734"></picture>

<!-- Окончание -->
<picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://user-images.githubusercontent.com/84059957/215088776-b06bbe95-42fd-4d78-bcae-70cdbeebbbd3.png">
    <source media="(prefers-color-scheme: light)" srcset="https://user-images.githubusercontent.com/84059957/210319906-4f1e79cb-1a45-4e5c-93e9-ae21e197e0b9.png">
    <img>
</picture>
</details>

3) для обхода траблов с CORS (wasm для получения html страницы порождает cors), использован [Cors Anywhere](https://cors-anywhere.herokuapp.com/) ([repo](https://github.com/Rob--W/cors-anywhere))

https://github.com/smir-ant/wasm_parser/blob/be879105deef195bbdbac1af26b46e0a2754fc02/src/pages/home.rs#L32

4) т.к. тут для обхода cors используется cors-anywhere, то перед тем как юзать статический скрипт надо запрос им кинуть на демо сюда https://cors-anywhere.herokuapp.com/corsdemo

5) github action (deploy.yaml) здесь нужен исключительно для автоматизации работы с github pages

