use gio::prelude::*;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Button, Entry, FileChooserAction, FileChooserDialog,
    Label, Orientation, ResponseType,
};
use std::fs::File;
use std::io::{BufRead, BufReader};
mod save_data;

fn main() {
    // Инициализация
    let app = Application::new(Some("com.kaznachey.gtk4"), gio::ApplicationFlags::FLAGS_NONE);

    app.connect_activate(master_ui);

    app.run();
}

fn master_ui(app: &Application) {
    // Основное окно
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Казначей альфа (демоверсия)"));
    window.set_default_size(400, 200);

    let vbox = gtk4::Box::new(Orientation::Vertical, 12);

    // Кнопка "Открыть файл"
    let hbox_choose_button = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
    let label_choose_button = Label::new(None);
    let choose_button = Button::with_label("Открыть файл");

    hbox_choose_button.append(&label_choose_button);
    hbox_choose_button.append(&choose_button);

    // Метка и поле ввода ФИО пользователя
    let hbox_klient = gtk4::Box::new(Orientation::Horizontal, 5);
    let label_klient = Label::new(Some("Имя:"));
    let intro_klient = Entry::new();

    hbox_klient.append(&label_klient);
    hbox_klient.append(&intro_klient);

    // ИНН ИП
    let hbox_inn = gtk4::Box::new(Orientation::Horizontal, 5);
    let label_inn = Label::new(Some("ИНН:"));
    let entry_inn = Entry::new();

    hbox_inn.append(&label_inn);
    hbox_inn.append(&entry_inn);

    // Метки и поля ввода для A (Доход) и B (Фикса)
    let hbox_label_a = gtk4::Box::new(Orientation::Horizontal, 5);
    let label_a = Label::new(Some("Доход:"));
    let entry_a = Entry::new();

    hbox_label_a.append(&label_a);
    hbox_label_a.append(&entry_a);

    let hbox_label_b = gtk4::Box::new(Orientation::Horizontal, 5);
    let label_b = Label::new(Some("Фикс:"));
    let entry_b = Entry::new();

    hbox_label_b.append(&label_b);
    hbox_label_b.append(&entry_b);

    // Метка Рассчитать
    let label_result = Label::new(Some(
        "Имя: \rИНН: \rБаза: \rНалог: \rСумма к выплате: \rFrom file:",
    ));

    // Тестирую вариант расчета данных из файла. Кнопка открытия файла.
    choose_button.connect_clicked(clone!(@strong window, @strong label_choose_button => move |_| {
        let file_chooser = FileChooserDialog::new(
            Some("Import file"),
            Some(&window),
            FileChooserAction::Open,
            &[("_Cancel", ResponseType::Cancel), ("_Open", ResponseType::Accept)],
        );

        file_chooser.connect_response(clone!(@strong label_choose_button => move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file_path) = dialog.file().and_then(|f| f.path()) {
                    match File::open(file_path) {
                        Ok(file) => {
                            let total: f64 = BufReader::new(file)
                                .lines()
                                .filter_map(Result::ok)
                                .filter_map(|line| {
                                    line.split_whitespace()
                                        .filter_map(|word| word.parse::<f64>().ok())
                                        .next()
                                })
                                .sum();
                            label_choose_button.set_text(&format!("Всего к оплате: {}", total));
                        }
                        Err(err) => {
                            eprintln!("Error opening file: {}", err);
                            label_choose_button.set_text("Error opening file");
                        }
                    }
                }
            }
            dialog.close();
        }));

        file_chooser.show();
    }));

    // Кнопка "Рассчитать"
    let calculate_button = Button::with_label("Рассчитать");
    calculate_button.connect_clicked(clone!(@strong intro_klient, @strong entry_inn, @strong entry_a, @strong entry_b, @strong label_result => move |_| {
        let result_inn = entry_inn.text().to_string();
        let result_klient = intro_klient.text().to_string();
        let a: f64 = entry_a.text().parse().unwrap_or(0.0);
        let b: f64 = entry_b.text().parse().unwrap_or(0.0);
        let baza_c = a - b;
        let nalog_d = baza_c * 0.06;
        let ceiled_nalog_d = nalog_d.ceil(); // Округление в большую сторону до целого числа
        let payment: f64 = nalog_d + b; // Сумма к выплате (Налог + Фикс)
        let ceiled_payment = payment.ceil(); // Округление ceil
        label_result.set_text(&format!("Имя: {} \rИНН: {} \rБаза: {} ₽\rНалог: {} ₽ \rСумма к выплате: {} ₽ \rPayment:", result_klient, result_inn, baza_c, ceiled_nalog_d, ceiled_payment));
    }));

    // Кнопка "Очистить"
    let clear_button = Button::with_label("Очистить");
    clear_button.connect_clicked(clone!(@strong entry_inn, @strong entry_a, @strong entry_b, @strong intro_klient, @strong label_result => move |_| {
        entry_inn.set_text("");
        entry_a.set_text("");
        entry_b.set_text("");
        intro_klient.set_text("");
        label_result.set_text("");
    }));

    // Кнопка "Сохранить файл"
    let save_button = Button::with_label("Сохранить файл");
    save_button.connect_clicked(clone!(@strong intro_klient, @strong label_result => move |_| {
        let text = label_result.text().as_str().to_string();
        save_data::save_to_file(&text, "Отчёт.txt").expect("Не удалось сохранить данные");
    }));

    // Добавляем все виджеты в контейнер
    vbox.append(&hbox_choose_button); // Теперь добавляем полностью hbox_choose_button
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
