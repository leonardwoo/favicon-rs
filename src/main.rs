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

extern crate image;

use std::collections::LinkedList;
use std::env;
use std::fs;
use std::path::Path;

use futures::executor::block_on;
use image::DynamicImage;
use image::imageops::{FilterType, resize};

/// --help
fn help() {
  println!("Usage: favicon -i <image.png> [output_pathname]
   -i <image.png>      input images with png
   [output_pathname]   output pathname
   -h                  display help for command
   -v                  output the version information
");
}

fn ehelp() {
  eprintln!("error: invalid command");
  help();
}

fn version() {
  let version = env!("CARGO_PKG_VERSION");
  println!("favicon {}", version);
}

struct ImgOpt {
  width: u32,
  height: u32,
  pathname: String,
}

/// image resize async executor
async fn executor(img: &DynamicImage, width: u32, height: u32, pathname: String) {
  let icon = resize(img, width, height, FilterType::Gaussian);
  icon.save(pathname).unwrap();
}

async fn generator_caller(src: &String, mut target: String) {
  if src.trim().is_empty() {
    eprintln!("Not found image");
    help();
  }

  let path = Path::new(&target);
  if !path.exists() {
    fs::create_dir_all(path).unwrap();
  }

  if cfg!(target_os = "windows") {
    if !target.ends_with("\\") {
      target = target + "\\";
    }
  } else {
    if !target.ends_with("/") {
      target = target + "/";
    }
  }

  let mut arr: LinkedList<ImgOpt> = LinkedList::new();

  // image size and name list start
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
  // image size and name list end

  let img = image::open(src).unwrap();
  for opt in arr.iter() {
    let pathname = target.clone() + &opt.pathname;
    executor(&img.clone(), opt.width, opt.height, pathname).await;
  }
}

fn generator(src: &String, target: String) {
  block_on(generator_caller(src, target));
  println!("Convert Completed");
}

fn main() {
  let args: Vec<String> = env::args().collect();
  // println!("{:?}", args);
  match args.len() {
    1 => {
      help();
    }
    2 => {
      let cmd = &args[1];
      match &cmd[..] {
        "-h" => help(),
        "-v" => version(),
        _ => {
          ehelp();
        }
      }
    }
    3 => {
      let cmd = &args[1];
      match &cmd[..] {
        "-i" => generator(&args[2], "./".to_string()),
        _ => {
          ehelp();
        }
      }
    }
    4 => {
      let cmd = &args[1];
      match &cmd[..] {
        "-i" => generator(&args[2], String::from(&args[3])),
        _ => {
          ehelp();
        }
      }
    }
    _ => {
      help();
    }
  }
}

