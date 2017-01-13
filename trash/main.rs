extern crate libc;

extern "C" {
  pub fn pugixml_version() -> i32;
}

pub fn version() -> i32
{
  unsafe {
    return pugixml_version();
  }
}

mod pugi {
  use std::ffi::CString;

  use std::ops::BitOr;

  use std::ptr::null;

  use std::default::Default;

  use std::ffi::CStr;

  use std::str::from_utf8;

}

fn main()
{
  match pugi::Document::new()
  {
    Ok(document) =>
          match document.load_file("cfe.xml",
                               vec![pugi::ParseOption::Default],
                               pugi::Encoding::Auto)
      {
        Ok(document) =>
        {
          println!("Passou!");
          let hue = document.save_file("cfe2.xml",
                                       "\thue",
                                       vec![pugi::FormatOption::Default],
                                       pugi::Encoding::Auto);
        }
        Err(why) => println!("Falhou: {:?}", why),
      }
    }
    Err(_) => println!("Falhou ao dar new"),
  }

  match pugi::Document::new()
  {
    Ok(document) =>
          match document.load_buffer("<xml></xml>",
                                 vec![pugi::ParseOption::Default],
                                 pugi::Encoding::Auto)
      {
        Ok(_) => println!("Passou!"),
        Err(why) => println!("Falhou: {:?}", why),
      }
    }
    Err(_) => println!("Falhou ao dar new"),
  }
  println!("{}", version())
}
