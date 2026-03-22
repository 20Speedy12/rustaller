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

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(TextView::new(
        "this is unfinished",
    ));

    
    siv.run()
}

fn addfile() {
//idk man do something with mtp funcs
}

fn addregkey(){
//michealsoft binbows
}

fn requirements(){
//i am a potato >:(
}

fn license(){
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::new()
            .title("enter absolute path to your license.txt file")
            
            .padding_lrtb(1, 1, 1, 0)
            .content(
                EditView::new()
                    .on_submit(show_popup)
                    .with_name("licensing")
                    .fixed_width(20),
            )
            .button("Ok", |s| {
 
                let licensing = s
                    .call_on_name("name", |view: &mut EditView| {
                        // We can return content from the closure!
                        view.get_content()
                    })
                    .unwrap();

                // Run the next step
                show_popup(s, &name);
            }),
    );

    siv.run();
//i love commiting tax evasion (joke)
}
