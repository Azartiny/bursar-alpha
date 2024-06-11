
use gio::prelude::*;
use glib::clone;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Button, Entry, FileChooserAction, FileChooserDialog, Label,
    Orientation, ResponseType,
};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::error::Error;
mod save_data;

fn main() {
    // Инициализация
    let app = Application::new(
        Some("com.kaznachey.gtk4"),
        gio::ApplicationFlags::FLAGS_NONE,
    );

    app.connect_activate(master_ui);

    app.run();
}

fn master_ui(app: &Application) {
    // Основное окно
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Казначей альфа (демоверсия)"));
    window.set_default_size(370, 200);

    let vbox = gtk4::Box::new(Orientation::Vertical, 12);

    // Кнопка "Открыть файл"
    let hbox_choose_button = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
    let label_choose_button = Label::new(None);
    let choose_button = Button::with_label("Открыть файл");

    hbox_choose_button.append(&label_choose_button);
    hbox_choose_button.append(&choose_button);

    // Метка и поле ввода ФИО пользователя
    let hbox_klient = gtk4::Box::new(Orientation::Horizontal, 5);
    let label_klient = Label::new(Some("  Имя:"));
    let intro_klient = Entry::new();

    hbox_klient.append(&label_klient);
    hbox_klient.append(&intro_klient);

    // ИНН ИП
    let hbox_inn = gtk4::Box::new(Orientation::Horizontal, 5);
    let label_inn = Label::new(Some("  ИНН:"));
    let entry_inn = Entry::new();

    hbox_inn.append(&label_inn);
    hbox_inn.append(&entry_inn);

    // Метки и поля ввода для A (Доход) и B (Фикса)
    let hbox_label_a = gtk4::Box::new(Orientation::Horizontal, 5);
    let label_a = Label::new(Some("  Доход:"));
    let entry_a = Entry::new();

    hbox_label_a.append(&label_a);
    hbox_label_a.append(&entry_a);

    let hbox_label_b = gtk4::Box::new(Orientation::Horizontal, 5);
    let label_b = Label::new(Some("  Фикс:"));
    let entry_b = Entry::new();

    hbox_label_b.append(&label_b);
    hbox_label_b.append(&entry_b);

    // Метка Рассчитать
    let label_result = Label::new(Some("Имя: \rИНН: \rБаза: \rНалог: \rСумма к выплате:"));

    // Функция удаления строк с определёнными ключевыми словами
    fn filter_lines(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let ignore_lines_words = vec![
            "Номер=", "Дата=", "ПлательщикСчет=", "ДатаСписано=", "Плательщик1=",
            "ПлательщикРасчСчет=", "ПлательщикБанк1=", "ПлательщикБанк2=", "ПлательщикБИК=",
            "ПлательщикКорсчет=", "ПолучательСчет=", "ДатаПоступило=", "Получатель1=",
            "ПолучательРасчСчет=", "ПолучательБанк1=", "ПолучательБанк2=", "ПолучательБИК=",
            "ПолучательКорсчет=", "ВидПлатежа=", "ВидОплаты=", "Код=", "ПолучательКПП=",
            "ПлательщикКПП=", "СрокПлатежа=", "Очередность=", "НазначениеПлатежа="
        ];

        let filtered_lines: Vec<String> = reader.lines()
            .filter_map(|line| {
                let line = line.ok()?;
                if ignore_lines_words.iter().any(|&word| line.starts_with(word)) {
                    None
                } else {
                    Some(line)
                }
            })
            .collect();

        Ok(filtered_lines)
    }

    // Функция обработки файла
    fn process_file(lines: Vec<String>, user_inn: &str) -> Result<f64, Box<dyn Error>> {
        let mut current_block: Vec<String> = Vec::new();
        let mut sum = 0.0;
        let mut ignore_block = false;
        let mut in_block = false;

        for line in lines {
            let line = line.trim().to_string();
            println!("Читаю строку: {}", &line); // Для отладки

            if line.starts_with("СекцияДокумент=") {
                in_block = true;
                current_block.clear();
                ignore_block = false;
                println!("Начало блока"); // Для отладки
            } else if line.starts_with("КонецДокумента") {
                in_block = false;
                println!("Конец блока"); // Для отладки

                if !ignore_block {
                    for item in &current_block {
                        if let Some(value) = item.strip_prefix("Сумма=") {
                            sum += value.trim().parse::<f64>()?;
                            println!("Добавил {} к общей сумме", value); // Для отладки
                        }
                    }
                }

                current_block.clear();
            }

            if in_block {
                current_block.push(line.clone());
                println!("Добавил в текущий блок: {}", &line); // Для отладки

                if line.starts_with("ПлательщикИНН=") || line.starts_with("ПолучательИНН=") {
                    let parts: Vec<&str> = line.split('=').collect();
                    if parts.len() == 2 && parts[1].trim() == user_inn {
                        let other_field = if line.starts_with("ПлательщикИНН=") {
                            "ПолучательИНН="
                        } else {
                            "ПлательщикИНН="
                        };

                        if let Some(other_inn_line) =
                            current_block.iter().find(|&l| l.starts_with(other_field))
                        {
                            let other_parts: Vec<&str> = other_inn_line.split('=').collect();
                            if other_parts.len() == 2 && other_parts[1].trim() == user_inn {
                                ignore_block = true;
                                println!("Игнорирую текущий блок"); // Для отладки
                            }
                        }
                    }
                }
            }
        }

        Ok(sum)
    }

    // Тестирую вариант расчета данных из файла. Кнопка открытия файла.
    choose_button.connect_clicked(clone!(@strong window, @strong label_choose_button, @strong entry_inn => move |_| {
        let user_inn = entry_inn.text().to_string();

        let file_chooser = FileChooserDialog::new(
            Some("Import file"),
            Some(&window),
            FileChooserAction::Open,
            &[("_Cancel", ResponseType::Cancel), ("_Open", ResponseType::Accept)],
        );

        file_chooser.connect_response(clone!(@strong label_choose_button, @strong user_inn => move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file_path) = dialog.file().and_then(|f| f.path()) {
                    let file_path = file_path.to_str().unwrap();
                    match filter_lines(file_path) {
                        Ok(filtered_lines) => {
                            match process_file(filtered_lines, &user_inn) {
                                Ok(sum) => {
                                    label_choose_button.set_text(&format!("{} ₽", sum));
                                },
                                Err(err) => {
                                    println!("Ошибка обработки файла: {}", err);
                                    label_choose_button.set_text("Ошибка обработки файла");
                                }
                            }
                        },
                        Err(err) => {
                            println!("Ошибка фильтрации строк: {}", err);
                            label_choose_button.set_text("Ошибка фильтрации строк");
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
    calculate_button.connect_clicked(clone!(@strong label_choose_button, @strong intro_klient, @strong entry_inn, @strong entry_a, @strong entry_b, @strong label_result => move |_| {
        let total_fromfile = label_choose_button.text().to_string();
        let result_inn = entry_inn.text().to_string();
        let result_klient = intro_klient.text().to_string();
        let a: f64 = entry_a.text().parse().unwrap_or(0.0);
        let b: f64 = entry_b.text().parse().unwrap_or(0.0);
        let baza_c = a - b;
        let nalog_d = baza_c * 0.06;
        let ceiled_nalog_d = nalog_d.ceil(); // Округление в большую сторону до целого числа
        let payment: f64 = nalog_d + b; // Сумма к выплате (Налог + Фикс)
        let ceiled_payment = payment.ceil(); // Округление ceil
        label_result.set_text(&format!("Имя: {} \rИНН: {} \rБаза: {} ₽ \rНалог: {} ₽ \rСумма к выплате: {} ₽ \rДанные из файла: {}", result_klient, result_inn, baza_c, ceiled_nalog_d, ceiled_payment, total_fromfile));
    }));

    // Кнопка "Очистить"
    let clear_button = Button::with_label("Очистить");
    clear_button.connect_clicked(clone!(@strong label_choose_button, @strong entry_inn, @strong entry_a, @strong entry_b, @strong intro_klient, @strong label_result => move |_| {
        label_choose_button.set_text("");
        entry_inn.set_text("");
        entry_a.set_text("");
        entry_b.set_text("");
        intro_klient.set_text("");
        label_result.set_text("");
    }));

    // Кнопка "Сохранить файл"
    let save_button = Button::with_label("Сохранить файл");
    save_button.connect_clicked(
        clone!(@strong label_choose_button, @strong intro_klient, @strong label_result => move |_| {
            let text = label_result.text().as_str().to_string();
            save_data::save_to_file(&text, "Отчёт.txt").expect("Не удалось сохранить данные");
        }),
    );

    // Добавляем все виджеты в контейнер
    vbox.append(&hbox_choose_button);
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