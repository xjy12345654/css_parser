// src/app.rs
slint::include_modules!();
use ::css_parser::{NoCssFilesFound, PaOptions, read_file};
use rfd::FileDialog;
use std::error::Error;
// use slint::ComponentHandle;
// use crate::AppWindow;
pub struct AppController {
    ui: AppWindow,
}

impl AppController {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let ui = AppWindow::new()?;
        Ok(Self { ui })
    }

    pub fn setup_event_handlers(&self) {
        {
            let ui_handle = self.ui.as_weak();
            // 按钮点击
            self.ui.on_btnclicked(move || {
                // let res= &ui_handle.unwrap();
                if let Some(ui) = ui_handle.upgrade() {
                    Self::handle_button_click(&ui);
                }
            });
        }

        {
            let ui_handle = self.ui.as_weak();
            // 文件选择
            self.ui.on_select_file(move || {
                if let Some(ui) = ui_handle.upgrade() {
                    Self::handle_file_selection(&ui);
                }
            });
        }
    }

    fn handle_button_click(ui: &AppWindow) {
        let checktype = ui.get_selected_index();
        let file_path: String = ui.invoke_getfilepath().into();
        // println!("file_path_{}", file_path);
        if file_path.is_empty() {
            ui.invoke_msgerr_pop(0);
            return;
        }

        let pa_option = if checktype == "0" {
            let str_num = ui.get_be_fontsize();
            let num: f32 = str_num.parse().unwrap_or(0.0);

            PaOptions {
                font_num: Some(num),
                checktype: Some(checktype.as_str()),
                file_path: Some(&file_path),
                ..Default::default()
            }
        } else {
            let be_width = ui.get_be_width();
            let be_height = ui.get_be_height();
            let be_width: f32 = be_width.parse().unwrap_or(0.0);
            let be_height: f32 = be_height.parse().unwrap_or(0.0);

            PaOptions {
                be_width: Some(be_width),
                be_height: Some(be_height),
                checktype: Some(checktype.as_str()),
                file_path: Some(&file_path),
                ..Default::default()
            }
        };

        let res = read_file(pa_option);
        match res {
            Ok(()) => {
                ui.invoke_msgsuc_pop();
            }
            Err(e) => {
                match e.downcast_ref::<NoCssFilesFound>() {
                    Some(_) => {
                        ui.invoke_msgerr_pop(2);
                        return;
                    }
                    _ => {}
                }
                ui.invoke_msgerr_pop(1);
            }
        };
    }

    fn handle_file_selection(ui: &AppWindow) {
        let selected_path = FileDialog::new().set_title("select FilePath").pick_folder();
        match selected_path {
            Some(path) => {
                let flie_path = path.display().to_string();
                ui.invoke_flie_inputte(flie_path.into());
            }
            None => {}
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        Ok(self.ui.run()?)
    }
}
