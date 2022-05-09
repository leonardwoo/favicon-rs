/*
 * Copyright (c) 2022 Leonard Woo
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 * list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 * this list of conditions and the following disclaimer in the documentation
 * and/or other materials provided with the distribution.
 *
 * 3. Neither the name of the copyright holder nor the names of its contributors
 * may be used to endorse or promote products derived from this software without
 * specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

use std::env;
use std::path::Path;
use std::fs;
use std::collections::LinkedList;

use futures::executor::block_on;

extern crate image;
use image::DynamicImage;
use image::imageops::{FilterType, resize};

fn help() {
  println!("usage:
favicon -i <image.png> [output_path]
");
}

struct ImgOpt {
  width: u32,
  height: u32,
  pathname: String,
}

async fn executor(img: &DynamicImage, width: u32, height: u32, pathname: String) {
  let icon = resize(img, width, height, FilterType::Gaussian);
  icon.save(pathname).unwrap();
}

async fn generator_caller(src: &String, target: String) {
  if src.trim().is_empty() {
    eprintln!("Not found image");
    help();
  }

  if !Path::new(&target).exists() {
    fs::create_dir_all(&target).unwrap();
  }

  let mut arr: LinkedList<ImgOpt> = LinkedList::new();

  arr.push_back(ImgOpt {
    width: 48,
    height: 48,
    pathname: "favicon.ico".to_string(),
  });
  arr.push_back(ImgOpt {
    width: 32,
    height: 32,
    pathname: "favicon-32x32.png".to_string(),
  });
  arr.push_back(ImgOpt {
    width: 192,
    height: 192,
    pathname: "android-chrome-192x192.png".to_string(),
  });
  arr.push_back(ImgOpt {
    width: 512,
    height: 512,
    pathname: "android-chrome-512x512.png".to_string(),
  });
  arr.push_back(ImgOpt {
    width: 180,
    height: 180,
    pathname: "apple-touch-icon.png".to_string(),
  });

  let img = image::open(src).unwrap();
  for opt in arr.iter() {
    let pathname = target.clone() + &opt.pathname;
    executor(&img.clone(), opt.width, opt.height, pathname).await;
  }

}

fn generator(src: &String, target: String) {
  block_on(generator_caller(src, target))
}

fn main() {
  let args: Vec<String> = env::args().collect();

  match args.len() {
    1 => {
    },
    2 => {
      let cmd = &args[1];
      match &cmd[..] {
        "-h" => help(),
        _ => {
          eprintln!("error: invalid command");
          help();
        },
      }
    },
    3 => {
      let cmd = &args[1];
      match &cmd[..] {
        "-i" => generator(&args[2], "./".to_string()),
        _ => {
          eprintln!("error: invalid command");
          help();
        },
      }

    },
    4 => {
      let cmd = &args[1];

      match &cmd[..] {
        "-i" => generator(&args[2], String::from(&args[3])),
        _ => {
          eprintln!("error: invalid command");
          help();
        },
      }

    },
    _ => {
      // show a help message
      help();
    }
  }

}

