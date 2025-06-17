// src/app.rs
slint::include_modules!();
use ::css_parser::{CSSError, PaOptions, read_file_ra};
use rfd::FileDialog;

use std::{error::Error, io::ErrorKind, thread, time::{Instant, Duration}};
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
        let file_path = ui.get_be_filepath().to_string();
        if file_path.is_empty() {
            ui.invoke_tip_msg(1, 1);
            return;
        }

        let str_num = ui.get_be_fontsize();
        let num: f32 = str_num.parse().unwrap_or(0.0);
        let be_width = ui.get_be_width();
        let be_height = ui.get_be_height();
        let be_width: f32 = be_width.parse().unwrap_or(0.0);
        let be_height: f32 = be_height.parse().unwrap_or(0.0);
        //启动等待弹层。
        ui.invoke_wait_popup_show();
        let re_ui = ui.as_weak();
        //简明处理没使用异步，使用多线程。操作文件开一个线程，保证等待弹层渲染。
        thread::spawn(move || {
            let pa_option = if checktype == "0" {
                PaOptions {
                    font_num: Some(num),
                    checktype: Some(checktype.as_str()),
                    file_path: Some(&file_path),
                    ..Default::default()
                }
            } else {
                PaOptions {
                    be_width: Some(be_width),
                    be_height: Some(be_height),
                    checktype: Some(checktype.as_str()),
                    file_path: Some(&file_path),
                    ..Default::default()
                }
            };
            //延时执行，否则处理少量文件看不到等待弹窗(doge)
            thread::sleep(Duration::from_millis(200));
            let start_time = Instant::now();
            let res = read_file_ra(pa_option);
            let elapsed = start_time.elapsed();
            println!("耗时___{:?}", elapsed);
            //该函数适合在线程处理完成任务时，把结果安全的传回ui线程 。
            slint::invoke_from_event_loop(move || {
                //关闭等待弹窗
                re_ui.unwrap().invoke_wait_popup_hide();
                match res {
                    Ok(()) => {
                        //处理成功后的轻提示回调
                        re_ui.unwrap().invoke_tip_msg(0, 0);
                    }
                    Err(e) => {
                        //处理失败后的轻提示回调
                        match e.downcast_ref::<CSSError>() {
                            Some(CSSError::NoCssFilesFound) => {
                                re_ui.unwrap().invoke_tip_msg(1, 3);
                            }
                            Some(CSSError::CSSSyntaxError) => {
                                re_ui.unwrap().invoke_tip_msg(1, 4);
                            }
                            _ => {}
                        }
                        //处理失败后的轻提示回调
                        if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                            // println!("erro__{}",io_err.kind());
                            match io_err.kind() {
                                ErrorKind::NotFound => re_ui.unwrap().invoke_tip_msg(1, 2),
                                ErrorKind::InvalidFilename => re_ui.unwrap().invoke_tip_msg(1, 5),
                                _ => {}
                            }
                        }
                    }
                };
            })
        });
    }
    // 设置文件路径函数
    fn handle_file_selection(ui: &AppWindow) {
        let selected_path = FileDialog::new().set_title("select FilePath").pick_folder();
        match selected_path {
            Some(path) => {
                let flie_path = path.display().to_string();
                ui.set_be_filepath(flie_path.into());
                // println!("{}", flie_path);
            }
            None => {}
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        Ok(self.ui.run()?)
    }
}
