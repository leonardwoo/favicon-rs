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

///
/// Author: Leonard Woo
///
use std::env;
use std::path::Path;
use std::fs;

extern crate image;
use image::imageops::FilterType;

fn help() {
  println!("usage:
favicon -i <image.png> [output_path]
");
}

fn generator(src: &String, target: String) {
  if src.trim().is_empty() {
    eprintln!("Not found image");
    help();
  }

  let img = image::open(src).unwrap();

  if !Path::new(&target).exists() {
    fs::create_dir_all(&target).unwrap();
  }

  let icon = image::imageops::resize(&img, 48, 48, FilterType::Gaussian);
  icon.save(target.clone() + "favicon.ico").unwrap();

  let icon32 = image::imageops::resize(&img, 32, 32, FilterType::Gaussian);
  icon32.save(target.clone() + "favicon-32x32.png").unwrap();

  let chrome192 = image::imageops::resize(&img, 192, 192, FilterType::Gaussian);
  chrome192.save(target.clone() + "android-chrome-192x192.png").unwrap();

  let chrome512 = image::imageops::resize(&img, 512, 512, FilterType::Gaussian);
  chrome512.save(target.clone() + "android-chrome-512x512.png").unwrap();

  let appletouch = image::imageops::resize(&img, 180, 180, FilterType::Gaussian);
  appletouch.save(target.clone() + "apple-touch-icon.png").unwrap();

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

