

use alloc::{vec::Vec, string::String, sync::Arc};
use embedded_graphics::{prelude::{Size, Point, RgbColor}, image::Image, text::Text, mono_font::{MonoTextStyle, iso_8859_13::FONT_6X12, ascii::FONT_10X20}, pixelcolor::Rgb888, Drawable};
use tinybmp::Bmp;

use crate::{sync::UPSafeCell, drivers::GPU_DEVICE};

use super::{Graphics, Component, ImageComp};

static FILEICON: &[u8] = include_bytes!("../assert/file.bmp");

pub struct IconController {
    inner: UPSafeCell<IconControllerInner>
}

pub struct IconControllerInner {
    files: Vec<String>,
    graphic: Graphics,
    parent: Option<Arc<dyn Component>>
}

impl IconController {
    pub fn new(files: Vec<String>,parent: Option<Arc<dyn Component>>) -> Self {
        IconController { 
            inner: unsafe {
                UPSafeCell::new(
                    IconControllerInner {
                        files,
                        graphic: Graphics { 
                            size: Size::new(1024, 768),
                            point: Point::new(0, 0), 
                            drv: GPU_DEVICE.clone()
                        },
                        parent,
                    }
                )
                
            }
        }
    }
}

impl Component for IconController {
    fn paint(&self) {
        println!("demo");
        let mut inner = self.inner.exclusive_access();
        let mut x = 10;
        let mut y = 10;
        let v = inner.files.clone();
        for file in v {
            println!("file");
            let bmp = Bmp::<Rgb888>::from_slice(FILEICON).unwrap();
                Image::new(&bmp, Point::new(x, y),).draw(&mut inner.graphic);
                let text = Text::new(
                    file.as_str(), 
                    Point::new(x + 20, y + 80), 
                    MonoTextStyle::new(&FONT_10X20,Rgb888::BLACK)
                );
                text.draw(&mut inner.graphic);
                if y >= 600 {
                    x = x + 70;
                    y = 10;
                } else {
                    y = y + 90;
                }
                
        }
    }

    fn add(&self, comp: Arc<dyn Component>) {
        todo!()
    }

    fn bound(&self) -> (Size, Point) {
        todo!()
    }
}