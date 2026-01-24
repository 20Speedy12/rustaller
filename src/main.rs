use cursive::{
    align::HAlign,
    event::{EventResult, Key},
    traits::With,
    view::{scroll::Scroller, Scrollable},
    views::{Dialog, OnEventView, Panel, TextView},
};
use std::fs;
use std::io;
use std::path::Path;
mod mtp;
//hi :3
fn main() {
    let mut siv = cursive::default();
    siv.add_layer(TextView::new(
        "Please type me for project\n\
         Press q to quit the application.",
    ));
    Dialog::new()
        .title("enter project name please")
    .content(
        .with_name("proj_name")
    )
    siv.add_global_callback('q',|s| s.quit());
    
    siv.run()
}
