use core::f64;
use gio::prelude::*;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, Entry, FileChooserAction, FileChooserDialog,
    Label, Orientation, ResponseType, Window,
};
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

mod save_data;

fn main() {
    //Инициализация
    let app = Application::new(
        Some("com.kaznachey.gtk4"),
        gio::ApplicationFlags::FLAGS_NONE,
    );

    app.connect_activate(master_ui);

    app.run();
}

fn master_ui(app: &Application) {
    //Основное окно
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Казначей альфа (демоверсия)"));
    window.set_default_size(400, 200);

    let vbox = Box::new(Orientation::Vertical, 12); //Вертикальный контейнер

    //Метка и поле ввода ФИО пользователя
    let hbox_klient = Box::new(Orientation::Horizontal, 5);
    let label_klient = Label::new(Some("Введите имя:"));
    let intro_klient = Entry::new();

    //ИНН ИП
    let hbox_inn = Box::new(Orientation::Horizontal, 5);
    let label_inn = Label::new(Some("ИНН:"));
    let entry_inn = Entry::new();

    //Метки и поля ввода для A (Доход) и B (Фикса)
    let hbox_label_a: Box = Box::new(Orientation::Horizontal, 5);
    let label_a = Label::new(Some("Введите Доход:"));
    let entry_a = Entry::new();

    let hbox_label_b: Box = Box::new(Orientation::Horizontal, 5);
    let label_b = Label::new(Some("Введите Фикс:"));
    let entry_b = Entry::new();

    //Меткa Рассчитать
    let label_result = Label::new(Some("Имя: \rИНН: \rБаза: \rНалог: \rСумма к выплате:"));

    //Кнопка Рассчитать
    let calculate_button = Button::with_label("Рассчитать");
    calculate_button.connect_clicked(clone!(@strong intro_klient, @strong entry_inn, @strong entry_a, @strong entry_b, @strong label_result => move |_| {
        let result_inn = entry_inn.text().to_string();
        let result_klient = intro_klient.text().to_string();
        let a: f64 = entry_a.text().parse().unwrap_or(0.0);
        let b: f64 = entry_b.text().parse().unwrap_or(0.0);
        let baza_c = a - b;
        let nalog_d = baza_c * 0.06;
        let ceiled_nalog_d = nalog_d.ceil(); // Округление в большую сторону до целого числа
        let payment: f64 = nalog_d + b; //Сумма к выплате (Налог + Фикса)
        let ceiled_payment = payment.ceil();//Округление ceil
        label_result.set_text(&format!("Имя: {} \rИНН: {} \rБаза: {} ₽\rНалог: {} ₽ \rСумма к выплате: {} ₽", result_klient, result_inn, baza_c, ceiled_nalog_d, ceiled_payment));
    }));

    //Кнопка очищения от всякого
    let clear_button = Button::with_label("Очистить");
    clear_button.connect_clicked(clone!(@strong entry_inn, @strong entry_a, @strong entry_b, @strong intro_klient, @strong label_result => move |_| {

        entry_inn.set_text("");
        entry_a.set_text("");
        entry_b.set_text("");
        intro_klient.set_text("");
        label_result.set_text("");
    }));

    //Кнопка сохранения данных
    let save_button = Button::with_label("Сохранить файл");
    save_button.connect_clicked(
        clone!(@strong intro_klient, @strong label_result => move |_| {
            let text = label_result.text().as_str().to_string();
            save_data::save_to_file(&text, "Отчёт.txt").expect("Не удалось сохранить данные");
        }),
    );

    // Добавляем все виджеты в контейнер

    hbox_klient.append(&label_klient);
    hbox_klient.append(&intro_klient);
    hbox_inn.append(&label_inn);
    hbox_inn.append(&entry_inn);
    hbox_label_a.append(&label_a);
    hbox_label_a.append(&entry_a);
    hbox_label_b.append(&label_b);
    hbox_label_b.append(&entry_b);

    vbox.append(&hbox_klient);
    vbox.append(&hbox_inn);
    vbox.append(&hbox_label_a);
    vbox.append(&hbox_label_b);
    vbox.append(&label_result);
    vbox.append(&calculate_button);
    vbox.append(&save_button);
    vbox.append(&clear_button);
    window.set_child(Some(&vbox));

    window.show();
}
