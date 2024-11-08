use std::io::{self, ErrorKind, Read};
use std::{thread, time::Duration};
use std::fs::{self, File};

fn main() {
    println!("Progam started.");
    if false {
      // 创建一个子线程
      let handle = thread::spawn(|| {
          // 这里模拟子线程中发生了某种错误，触发 panic
          thread::sleep(Duration::from_secs(1));
          // panic!("Something went wrong in the thread!");
      });

      // 等待子线程结束
      let result = handle.join();

      // 检查子线程的结果
      match result {
          Ok(_) => println!("Thread completed successfully."),
          Err(e) => println!("Thread panicked with error: {:?}", e),
      }

      // 主线程继续执行
      println!("Continuing execution in main thread.");
    }

    if false {
      let f = File::open("24.py");
      let mut result = match f {
          Ok(result) => result,
          Err(err) => match err.kind() {
            ErrorKind::NotFound => {
              panic!("File not found");
            },
            ErrorKind::PermissionDenied => {
              panic!("Permission denied.");
            },
            _ => {
              panic!("Can not open file.");
            },
          }
      };
      let mut buf = String::new();
      match result.read_to_string(&mut buf) {
        Ok(size) => {
          println!("Read {size} bytes: {buf}");
        },
        Err(err) => panic!("Read content failed, {err}"),
      };
    }
    
    fn read_file() -> Result<String, io::Error> {
      let mut f = File::open("24.py")?;
      let mut buf = String::new();
      f.read_to_string(&mut buf)?;
      Ok(buf)
    }
    fn read2() -> Result<String, io::Error> {
      let mut s = String::new();
      File::open("24.py")?.read_to_string(&mut s)?;
      Ok(s)
    }
    if let Ok(s) = read_file() {
      println!("file content = {}", s);
    };
    let Ok(s) = read2() else {
      panic!("can not open file");
    };
    println!("s = {s}");
    if let Ok(s) = fs::read_to_string("24.py") {
      println!("s = {}", s);
    };


}
