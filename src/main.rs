// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
mod app;
use crate::app::AppController;

fn main() -> Result<(), Box<dyn Error>> {
 
    // let ui = AppWindow::new()?;
    // let window = ui.window();
    // println!("{:?}", window.position());
    // println!("{:?}", window.scale_factor());
    // window.set_position(slint::PhysicalPosition::new(500, 260));

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });
    // ui.run()?;
    //    let res= ui.on_checkclicked({
    //         let ui_handle = ui.as_weak();
    //         move |id, _| {
    //             let ui = ui_handle.unwrap();
    //             let str_num = ui.get_checklist();
    //             println!("---{}", id);
    //             for mut item in str_num.iter() {
    //                 //    println!("{}",id);
    //                 //    println!("{}",item.check_type);
    //                 //    if(item)
    //                 item.check_type = true;
    //                 // if id == item.id {
    //                 //     item.check_type =true;
    //                 // }
    //                 println!("{:?}",item.check_type);
    //             }

    //         }
    //     });

    // ui.on_btnclicked({
    //     let ui_handle = ui.as_weak();
    //     move || {

    //         let ui = ui_handle.unwrap();
    //         let checktype = ui.get_selected_index();
    //         let file_path: String = ui.invoke_getfilepath().into();
    //         if file_path.is_empty() {
    //             // println!("文件路径为空_{}", file_path);
    //             ui.invoke_msgerr_pop(0);
    //             return;
    //         }
    //         // println!("path_{}", file_path);
    //         let pa_option = if checktype == "0" {
    //             let str_num = ui.get_be_fontsize();
    //             let num: f32 = str_num.parse().unwrap();
    //             println!("{}", num);
    //             PaOptions {
    //                 font_num: Some(num),
    //                 checktype: Some(checktype.as_str()),
    //                 file_path: Some(&file_path),
    //                 ..Default::default()
    //             }
    //         } else {
    //             let be_width = ui.get_be_width();
    //             let be_height = ui.get_be_height();
    //             let be_width: f32 = be_width.parse().unwrap();
    //             let be_height: f32 = be_height.parse().unwrap();

    //             PaOptions {
    //                 be_width: Some(be_width),
    //                 be_height: Some(be_height),
    //                 checktype: Some(checktype.as_str()),
    //                 file_path: Some(&file_path),
    //                 ..Default::default()
    //             }

    //             // let _ = read_file(pa_option);
    //         };
    //         let _res: Result<(), Box<dyn Error>> = read_file(pa_option);

    //         match _res {
    //             Ok(()) => {
    //                 ui.invoke_msgsuc_pop();
    //             }
    //             Err(e) => {
    //                 // println!("{}", e);
    //                 match e.downcast_ref::<NoCssFilesFound>() {
    //                     Some(_e) => {
    //                         ui.invoke_msgerr_pop(2);
    //                         return;
    //                     }
    //                     _ => {}
    //                 }
    //                 ui.invoke_msgerr_pop(1);
    //             }
    //         };
    //     }
    // });

    // ui.on_select_file({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         let selected_path = FileDialog::new().set_title("select FilePath").pick_folder();
    //         match selected_path {
    //             Some(path) => {
    //                 // println!("选择的文件夹路径: {}", path.display());
    //                 let flie_path = path.display().to_string();
    //                 ui.invoke_flie_inputte(flie_path.into());
    //             }
    //             None => {
    //                 // println!("取消了选择");
    //             }
    //         }
    //     }
    // });

    // ui.run()?;
    // Ok(())
    let app = AppController::new()?;
    app.setup_event_handlers();
    app.run()
   
}
