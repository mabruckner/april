
#[cfg(feature = "image")]
extern crate image;

#[cfg(feature = "image")]
use image::{GenericImage, Luma};


pub mod bindings;

#[derive(Debug, Copy, Clone)]
pub enum TagFamilyType {
    Tag16h5,
    Tag25h7,
    Tag25h9,
    Tag36h10,
    Tag36h11
}

pub struct TagFamily {
    family_type: TagFamilyType,
    ptr: *mut bindings::apriltag_family_t
}

impl TagFamily {
    fn create(family: TagFamilyType) -> Self {
        let ptr = unsafe {
            use bindings::*;
            use TagFamilyType::*;
            match family {
                Tag16h5 => tag16h5_create(),
                Tag25h7 => tag25h7_create(),
                Tag25h9 => tag25h9_create(),
                Tag36h10 => tag36h10_create(),
                Tag36h11 => tag36h11_create()
            }
        };
        TagFamily {
            family_type: family,
            ptr: ptr
        }
    }
}

impl From<TagFamilyType> for TagFamily {
    fn from(family: TagFamilyType) -> TagFamily {
        TagFamily::create(family)
    }
}

impl Drop for TagFamily {
    fn drop(&mut self) {
        use bindings::*;
        use TagFamilyType::*;
        unsafe {
            match self.family_type {
                Tag16h5 => tag16h5_destroy(self.ptr),
                Tag25h7 => tag25h7_destroy(self.ptr),
                Tag25h9 => tag25h9_destroy(self.ptr),
                Tag36h10 => tag36h10_destroy(self.ptr),
                Tag36h11 => tag36h11_destroy(self.ptr)
            }
        }
    }
}


pub struct Detector {
    ptr: *mut bindings::apriltag_detector_t,
    families: Vec<TagFamily>
}

impl Detector {
    pub fn create() -> Detector {
        unsafe {
            let ptr = bindings::apriltag_detector_create();
            println!("decimate: {:?}", (*ptr).quad_decimate);
            Detector {
                ptr: ptr,
                families: Vec::new()
            }
        }
    }
    pub fn add_family(&mut self, family: TagFamilyType) -> (){
        let family: TagFamily = family.into();
        unsafe {
            bindings::apriltag_detector_add_family_bits(self.ptr, family.ptr, 1);
        }
        self.families.push(family);
    }

    pub fn detect(&self, image: &mut Image) -> Detections {
        unsafe {
            let mut img = image.as_binding_image();
            Detections {
                array: bindings::apriltag_detector_detect(self.ptr, &mut img)
            }
        }
    }
}

impl Drop for Detector {
    fn drop(&mut self) -> () {
        unsafe {
            bindings::apriltag_detector_destroy(self.ptr);
        }
    }
}

pub struct Detections {
    array: *mut bindings::zarray
}

impl Detections {
    pub fn len(&self) -> usize {
        unsafe {
            (*self.array).size as usize
        }
    }
    pub fn get<'a>(&'a self, idx: usize) -> Detection<'a> {
        unsafe {
            Detection {
                life: std::marker::PhantomData,
                item: *((*self.array).data.offset((*self.array).el_sz as isize * idx as isize) as *mut *mut bindings::apriltag_detection)
            }
        }
    }
}


impl Drop for Detections {
    fn drop(&mut self) {
        unsafe {
            bindings::apriltag_detections_destroy(self.array);
        }
    }
}

pub struct Detection<'a> {
    life: std::marker::PhantomData<&'a ()>,
    item: *mut bindings::apriltag_detection
}

impl <'a> Detection<'a> {
    pub fn id(&self) -> usize {
        unsafe {
            (*self.item).id as usize
        }
    }
    pub fn goodness(&self) -> f32 {
        unsafe {
            (*self.item).goodness
        }
    }
    pub fn margin(&self) -> usize {
        unsafe {
            (*self.item).decision_margin as usize
        }
    }
    pub fn c(&self) -> [f64; 2] {
        unsafe {
            (*self.item).c
        }
    }
    pub fn p(&self) -> [[f64; 2]; 4] {
        unsafe {
            (*self.item).p
        }
    }
}

#[derive(Debug)]
pub struct Image {
    pub buf: Vec<u8>,
    pub width: usize,
    pub height: usize
}

impl Image {
    pub fn create(buf: Vec<u8>, width:usize, height:usize) -> Image {
        Image {
            buf: buf,
            width: width,
            height: height
        }
    }

    unsafe fn as_binding_image(&mut self) -> bindings::image_u8 {
        bindings::image_u8 {
            buf: self.buf.as_mut_ptr(),
            width: self.width as i32,
            height: self.height as i32,
            stride: self.width as i32
        }
    }
}



#[cfg(feature = "image")]
impl  <T:GenericImage<Pixel=Luma<u8>>> From<T> for Image {
    fn from(img: T) -> Image {
        Image {
            buf: img.pixels().map(|(_, _, p)| p.data[0]).collect(),
            width: img.width() as usize,
            height: img.height() as usize,
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
