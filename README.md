# BambangShop Receiver App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a Rocket web framework skeleton that you can work with.

As this is an Observer Design Pattern tutorial repository, you need to implement a feature: `Notification`.
This feature will receive notifications of creation, promotion, and deletion of a product, when this receiver instance is subscribed to a certain product type.
The notification will be sent using HTTP POST request, so you need to make the receiver endpoint in this project.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Receiver" folder.

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
    ```
    Here are the details of each environment variable:
    | variable                | type   | description                                                     |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT             | string | Port number that will be listened by this receiver instance.    |
    | APP_INSTANCE_ROOT_URL   | string | URL address where this receiver instance can be accessed.       |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed.       |
    | APP_INSTANCE_NAME       | string | Name of this receiver instance, will be shown on notifications. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)
3.  To simulate multiple instances of BambangShop Receiver (as the tutorial mandates you to do so),
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:
    -   Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    -   Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    -   Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.

## Mandatory Checklists (Subscriber)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create SubscriberRequest model struct.`
    -   [ ] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Notification repository.`
    -   [ ] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Commit: `Implement receive_notification function in Notification service.`
    -   [ ] Commit: `Implement receive function in Notification controller.`
    -   [ ] Commit: `Implement list_messages function in Notification service.`
    -   [ ] Commit: `Implement list function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1

1. In this tutorial, we used RwLock<> to synchronise the use of Vec of Notifications. Explain why it is necessary for this case, and explain why we do not use Mutex<> instead?

    Jawab:

    Dalam tutorial ini, penggunaan RwLock<> untuk menyinkronkan penggunaan Vec dari Notifikasi diperlukan karena kita ingin memastikan akses ke Vec tersebut dapat dilakukan secara aman oleh banyak thread secara konkuren. RwLock<> memungkinkan multiple reader (pembaca) atau satu writer (penulis) pada satu waktu. Dalam konteks tutorial ini, kita ingin memungkinkan banyak thread untuk membaca data notifikasi yang disimpan dalam Vec, sementara hanya satu thread yang dapat menulis (menambah atau menghapus) data notifikasi. Oleh karena itu, RwLock<> cocok digunakan untuk kasus ini.

    Mengapa tidak menggunakan Mutex<>? Mutex<> akan membatasi akses ke data notifikasi menjadi satu thread pada satu waktu, baik untuk operasi membaca maupun menulis. Ini akan mengakibatkan kinerja yang buruk terutama dalam skenario di mana banyak thread hanya ingin membaca data notifikasi tanpa melakukan operasi penulisan. RwLock<> memberikan fleksibilitas dengan memungkinkan multiple reader sekaligus satu writer, yang lebih sesuai dengan kebutuhan dalam kasus ini.

2. In this tutorial, we used lazy_static external library to define Vec and DashMap as a “static” variable. Compared to Java where we can mutate the content of a static variable via a static function, why did not Rust allow us to do so?

    Jawab:

    Dalam Rust, variabel statis dideklarasikan dengan kata kunci "static". Secara default, variabel statis bersifat immutable (tidak dapat diubah), dan Rust tidak mengizinkan perubahan nilai variabel statis setelah deklarasi. Ini adalah keputusan desain yang sengaja dibuat untuk mencegah masalah thread-safety yang mungkin timbul akibat perubahan nilai variabel statis dari berbagai thread secara konkuren. Rust mendorong paradigma pemrograman yang aman secara thread, dan membatasi mutasi variabel statis adalah salah satu cara untuk mencapai tujuan tersebut.

    Dalam Java, meskipun Anda dapat mengubah konten dari variabel statis melalui fungsi statis, hal ini dapat menyebabkan masalah konkurensi jika tidak dilakukan dengan hati-hati. Java menyediakan mekanisme untuk mengunci variabel statis menggunakan kata kunci "synchronized" atau "volatile" untuk memastikan akses yang aman dari thread yang berbeda. Dalam Rust, pendekatan yang berbeda digunakan dengan menggunakan RwLock<> atau Mutex<> untuk menyinkronkan akses ke variabel statis saat diperlukan.

#### Reflection Subscriber-2

1. Have you explored things outside of the steps in the tutorial, for example: src/lib.rs? If not, explain why you did not do so. If yes, explain things that you have learned from those other parts of code.

    Jawab:
    
    Saya menjelajahi kode di luar langkah-langkah tutorial untuk memahami bagaimana komponen-komponen sistem notifikasi terstruktur dan bekerja. Dengan begitu saya mendapatkan wawasan tentang bagaimana pola Observer diimplementasikan, bagaimana notifikasi ditangani, dan bagaimana Vec dari notifikasi disinkronkan menggunakan RwLock.

2. Since you have completed the tutorial by now and have tried to test your notification system by spawning multiple instances of Receiver, explain how Observer pattern eases you to plug in more subscribers. How about spawning more than one instance of Main app, will it still be easy enough to add to the system?

    Jawab:

    Terkait pengujian sistem notifikasi dengan memunculkan beberapa instans dari Receiver, saya menemukan bahwa pola Observer memang memudahkan proses penambahan lebih banyak subscriber. Dengan hanya membuat instans baru dari Receiver dan mendaftarkannya sebagai subscriber ke Publisher, saya dapat dengan mudah memperluas sistem untuk menampung receiver tambahan tanpa harus mengubah logika inti dari sistem.

    Mengenai memunculkan lebih dari satu instans dari Main app, masih relatif mudah untuk menambahkannya ke dalam sistem. Karena Publisher menangani distribusi notifikasi kepada semua subscriber yang terdaftar, menambahkan lebih banyak instans dari Main app tidak akan memerlukan perubahan yang signifikan pada kode yang sudah ada. Setiap instans dari Main app akan bertindak sebagai subscriber independen, menerima notifikasi saat dipublikasikan oleh Publisher.

3. Have you tried to make your own Tests, or enhance documentation on your Postman collection? If you have tried those features, tell us whether it is useful for your work (it can be your tutorial work or your Group Project).

    Jawab:

    Dalam hal pengujian, saya melakuakan test untuk memvalidasi fungsionalitas sistem notifikasi. Selain itu, saya juga meningkatkan dokumentasi pada koleksi Postman saya dengan memberikan deskripsi yang detail untuk setiap endpoint API, termasuk format permintaan yang diharapkan dan struktur respons. Fitur-fitur ini berguna baik untuk pekerjaan tutorial maupun proyek kelompok saya, karena membantu memastikan kebenaran dan kehandalan dari fungsionalitas yang diimplementasikan.