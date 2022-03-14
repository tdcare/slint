// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial
use slint::Model;
use std::rc::Rc;

slint::slint!(import { MainWindow } from "crud.slint";);

pub fn main() {
    let main_window = MainWindow::new();

    let model = Rc::new(slint::VecModel::<slint::StandardListViewItem>::from(vec![
        "Emil, Hans".into(),
        "Mustermann, Max".into(),
        "Tisch, Roman".into(),
    ]));
    main_window.set_names_list(model.clone().into());

    {
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        main_window.on_createClicked(move || {
            let main_window = main_window_weak.unwrap();
            let new_entry = main_window.get_surname() + ", " + main_window.get_name().as_str();

            model.push(new_entry.into());
        });
    }

    {
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        main_window.on_updateClicked(move || {
            let main_window = main_window_weak.unwrap();
            let index = main_window.get_current_item() as usize;
            let entry = main_window.get_surname() + ", " + main_window.get_name().as_str();

            model.set_row_data(index, entry.into());
        });
    }

    {
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        main_window.on_deleteClicked(move || {
            let main_window = main_window_weak.unwrap();
            let index = main_window.get_current_item() as usize;

            model.remove(index);
        });
    }

    main_window.run();
}