//! Junk. Testing subprocess execution.

use async_std::prelude::*;
//use async_macros::join;
use std::future;
use std::io::{BufRead, Read};
use std::process::Stdio;

#[cfg(target_os = "macos")]
static EXE_TO_RUN: &str = "./repeat";
#[cfg(target_os = "linux")]
static EXE_TO_RUN: &str = "./repeat";
#[cfg(target_os = "windows")]
static EXE_TO_RUN: &str = "repeat.exe";

pub fn test_subprocesses() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = std::sync::mpsc::channel();
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    println!("Start bytes thread a");
    println!("{}", std::path::Path::new(EXE_TO_RUN).exists());
    {
        //let child1 = std::process::Command::new(EXE_TO_RUN).output().expect("ttttttttrrr");
        //println!("{}", String::from_utf8(child1.stdout).unwrap());
        //return Ok(());
        let child = std::process::Command::new(EXE_TO_RUN)
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn process.");
        let stdout = child
            .stdout
            //.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "bytes"))?;
            .expect("child has stdout.");
        println!("Start bytes thread");
        std::thread::spawn(move || {
            stdout.bytes().filter_map(|b| b.ok()).for_each(|b| {
                tx.send(String::from_utf8(vec![b]).unwrap())
                    .expect("send byes on channel.");
            });
        });
    }

    println!("Start lines thread!");
    {
        let stdout = std::process::Command::new(EXE_TO_RUN)
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "lines"))?;
        let reader = std::io::BufReader::new(stdout);
        std::thread::spawn(move || {
            reader
                .lines()
                .filter_map(|line| line.ok())
                //.filter(|line| line.find("<").is_some())
                //.for_each(|line| println!("{}", line));
                .for_each(|line| {
                    tx2.send(line).expect("send line on channel.");
                });
        });
    }

    let mut count = 0;
    while count < 40 {
        println!("{:?}", rx.recv().unwrap());
        std::thread::sleep(std::time::Duration::from_millis(250));
        count += 1;
    }
    Ok(())
}

pub async fn test_async_subprocesses() -> Result<(), Box<dyn std::error::Error>> {
    println!("Create buf reader future");
    let stdout = std::process::Command::new(EXE_TO_RUN)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "bytes"))?;
    println!("aaaaaaaaaaaaaaaaaaaaaa");
    let reader = async_std::io::BufReader::new(futures::io::AllowStdIo::new(stdout));
    let future1 = reader.bytes().filter_map(|b| b.ok()).for_each(|b| {
        println!("{}", String::from_utf8(vec![b]).unwrap());
    });

    println!("Create buf reader future2");
    let stdout2 = std::process::Command::new(EXE_TO_RUN)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "bytes"))?;
    println!("aaaaaaaaaaaaaaaaaaaaaa");
    let reader2 = async_std::io::BufReader::new(futures::io::AllowStdIo::new(stdout2));
    let future2 = reader2.lines().filter_map(|b| b.ok()).for_each(|b| {
        println!("{}", b);
    });

    //async_std::futures::join!(future1, future2);
    futures::join!(future1, future2);
    //future1.join(future2).await;
    //future1.await;
    //future2.await;
    println!("bbbbbbbbbbbbbbbbbbbbbbb");

    Ok(())
}
