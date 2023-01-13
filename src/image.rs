// An RGB image as a flat vector.
// Pixels are contained row by row, each pixel is represented by 3 bytes.
pub struct FlatImg {
	pub width: usize,
	pub height: usize,
	pub img: Vec<u8>,
}


impl FlatImg {
	pub fn from_2d_vec<T: ToNum>(v: &Vec<Vec<T>>) -> FlatImg {
		let n = v.len();
		let m = v[0].len();
		let mut img = vec![0; n*m*3];
		for i in 0..n {
			for j in 0..m {
				let rgb = num_to_rgb(v[i][j].to_num());
				let index = (i*m + j) * 3;
				img[index] = rgb[0];
				img[index + 1] = rgb[1];
				img[index + 2] = rgb[2];
			}
		}
		FlatImg {
			img: img,
			width: m,
			height: n,
		}
	}
}
// Any 2-dim vectors of a type T implementing this trait can be converted
// into a RGB image.
// A value of type T is first converted to a u64, which is then converted
// to a RGB value using the num_to_rgb function.
pub trait ToNum {
	fn to_num(&self) -> u64;
}

impl ToNum for u8 {
	fn to_num(&self) -> u64 {
		*self as u64
	}
}

fn num_to_rgb(t: u64) -> [u8; 3] {
    match t {
        0 => [255,255,255],
        1 => [0,0,0],
        2 => [255,0,0],
        3 => [0,255,0],
        4 => [0,0,255],
        5 => [255,255,0],
        6 => [0,255,255],
        7 => [128,0,0],
        8 => [128,128,0],
        9 => [128,0,128],
        _ => [0,0,0],
    }
}

// Returns the width and height of an image with (width, height)=(iw, ih)
// fitted into a frame with (width, height)=(fw, fh).
pub fn fit_image_size(fw: f32, fh: f32, iw: f32, ih: f32) -> (f32, f32) {
	// scale to width
	let height = ih * fw / iw;
	if height <= fh {
		return (fw, height);
	}

	// scale to height
	let width = iw * fh / ih;
	return (width, fh);
} 