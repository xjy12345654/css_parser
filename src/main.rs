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

    

    // ui.run()?;
    // Ok(())
    let app = AppController::new()?;
    app.setup_event_handlers();
    app.run()
   
}
