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
//i love commiting tax evasion (joke)
}
