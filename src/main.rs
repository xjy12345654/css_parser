// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
slint::include_modules!();
use ::css_parser::{PaOptions, read_file};
fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
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

    ui.on_btnclicked({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let checktype = ui.get_selected_index();
            let pa_option = if checktype == "0" {
                // println!("rem");
                let str_num = ui.get_be_fontsize();
                let num: f32 = str_num.parse().unwrap();
                PaOptions {
                    font_num: Some(num),
                    checktype: Some(checktype.as_str()),
                    ..Default::default()
                }
                // let _ = read_file(pa_option);
            } else {
                let be_width = ui.get_be_width();
                let be_height = ui.get_be_height();
                let be_width: f32 = be_width.parse().unwrap();
                let be_height: f32 = be_height.parse().unwrap();
                // println!("{}",be_height);
                // println!("vw");
                PaOptions {
                    be_width: Some(be_width),
                    be_height: Some(be_height),
                    checktype: Some(checktype.as_str()),
                    ..Default::default()
                }

                // let _ = read_file(pa_option);
            };
            let _res: Result<(), Box<dyn Error>> = read_file(pa_option);

            match _res {
                Ok(()) => {
                    ui.invoke_msgsuc_pop();
                }
                Err(_e) => {
                    // ui.set_msgerr_pop_opacity_flag(true);
                    ui.invoke_msgerr_pop();
                }
            };
        }
    });

    ui.run()?;
    Ok(())
}
