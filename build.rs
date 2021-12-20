#[cfg(windows)]
use winres;

#[cfg(windows)]
fn main() {
    if cfg!(target_os = "windows") {
      let mut res = winres::WindowsResource::new();
      res.set_icon("./drivers/app_image_icon_oUt_icon.ico");
      res.compile().unwrap();
    }
}