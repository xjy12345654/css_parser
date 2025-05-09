use std::error::Error;
use ::css_parser::{NoCssFilesFound, PaOptions, read_file};
use rfd::FileDialog;
use slint::ComponentHandle;
use crate::AppWindow;
pub fn setup_btn_clicked(ui:  &AppWindow) {
    ui.on_btnclicked({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let checktype = ui.get_selected_index();
            let file_path: String = ui.invoke_getfilepath().into();
            if file_path.is_empty() {
                ui.invoke_msgerr_pop(0);
                return;
            }
            
            let pa_option = if checktype == "0" {
                let str_num = ui.get_be_fontsize();
                let num: f32 = str_num.parse().unwrap();
                
                PaOptions {
                    font_num: Some(num),
                    checktype: Some(checktype.as_str()),
                    file_path: Some(&file_path),
                    ..Default::default()
                }
            } else {
                let be_width = ui.get_be_width();
                let be_height = ui.get_be_height();
                let be_width: f32 = be_width.parse().unwrap();
                let be_height: f32 = be_height.parse().unwrap();

                PaOptions {
                    be_width: Some(be_width),
                    be_height: Some(be_height),
                    checktype: Some(checktype.as_str()),
                    file_path: Some(&file_path),
                    ..Default::default()
                }
            };
            
            let _res: Result<(), Box<dyn Error>> = read_file(pa_option);

            match _res {
                Ok(()) => {
                    ui.invoke_msgsuc_pop();
                }
                Err(e) => {
                    match e.downcast_ref::<NoCssFilesFound>() {
                        Some(_e) => {
                            ui.invoke_msgerr_pop(2);
                            return;
                        }
                        _ => {}
                    }
                    ui.invoke_msgerr_pop(1);
                }
            };
        }
    });
}

pub fn setup_select_file(ui: &AppWindow) {
    ui.on_select_file({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let selected_path = FileDialog::new().set_title("select FilePath").pick_folder();
            match selected_path {
                Some(path) => {
                    let file_path = path.display().to_string();
                    ui.invoke_flie_inputte(file_path.into());
                }
                None => {
                    // 用户取消选择
                }
            }
        }
    });
}