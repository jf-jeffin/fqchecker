pub mod fqchecker;
mod tui;

use fqchecker::fqc::*;
// use std::sync::Arc;
// use std::sync::Mutex;

// use std::fs::File;
// use std::io::{BufReader};
// use std::process::exit;
// use tokio::runtime::Runtime;
    use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::watch;
use tokio::time::sleep;
use crate::tui::Interface;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let file_path = "C:\\Users\\Jef Finn\\Downloads\\downsampled_SampleB_R1.fastq\\downsampled_SampleB_R1.fastq";
    let mut prog_stat =CrrFileProcessInfo::new(file_path);
    let mut r = CrrFileProcessInfo::new(file_path);

    let (send_port, receive_port) = watch::channel(prog_stat);
    let mut tui_if = Interface::default();
    // tokio::spawn(async move {
    //     match tui::exec(&mut tui_if,receive_port){
    //         Ok(_) => {},
    //         Err(e) => {println!("wwwwwwwwwwwwwwwwwwwwwwww{}",e)}
    //     };
    // });
    tokio::task::spawn_blocking(move || {
        tui::exec(&mut tui_if, receive_port).unwrap();
    });


    fq_init(&mut r, send_port);
    Ok(())


}
    // let info = Arc::new(Mutex::new(tui::Interface::default().set_file_name(file_path)));
    // let info_2 = info.clone();
    // let _ =tokio::task::spawn_blocking(move ||   {
    //      loop
    //      {
    //         let mut guard = info.lock().unwrap();
    //          // {
    //         //     Ok(guard) => guard,
    //         //     Err(_)=> panic!("Couldn't lock mutex"),
    //         // };
    //         fq_init(&mut guard.info);
    //         // drop(guard);
    //          std::thread::sleep(Duration::from_millis(1));
    //         // sleep(Duration::from_millis(500));
    //      }
    // });
    //     let _ =tui::exec(info_2);
    //
    // }




// async fn maain() -> std::io::Result<()> {
//     // let mut infos = CrrFileProcessInfo::new("C:\\Users\\Jef Finn\\Downloads\\downsampled_SampleB_R1.fastq\\downsampled_SampleB_R1.fastq");
//     let mut r = Arc::new(Mutex::new(tui::Interface::default()));
//     // r.file_name = "C:\\Users\\Jef Finn\\Downloads\\downsampled_SampleB_R1.fastq\\downsampled_SampleB_R1.fastq".to_string();
//     r.info = CrrFileProcessInfo::new("C:\\Users\\Jef Finn\\Downloads\\downsampled_SampleB_R1.fastq\\downsampled_SampleB_R1.fastq");
//
//     // loop {
//         tokio::task::spawn_blocking(|| {
//             {
//                 let _ = tui::exec(&mut r);
//             }
//         }).await?;
//     fq_init(&mut r.info);
//     // }
//     Ok(())
// }


    // let file = File::open("C:\\Users\\Jef Finn\\Downloads\\downsampled_SampleB_R1.fastq\\downsampled_SampleB_R1.fastq");
    // match file {
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         exit(1)
    //     }
    //     Ok(f) => {
    //         let input_file_reader = BufReader::new(f);
    //         let g = extract_count(input_file_reader);
    //         g.0.dsply_table();
    //         g.0.plot_graph();
    //         g.1.dsply_table();

            // calculate_metrics(&g);
    //     }
    // }





