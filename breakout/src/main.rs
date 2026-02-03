use citro2d::Instance;
use citro2d_sys::{
	C2D_DrawParams, C2D_DrawParams__bindgen_ty_1, C2D_DrawParams__bindgen_ty_2, C2D_Image,
	C2D_Sprite, C3D_Tex, C3D_Tex__bindgen_ty_1, C3D_Tex__bindgen_ty_2, C3D_Tex__bindgen_ty_3,
	Tex3DS_SubTexture,
};
use citro3d_sys;
use ctru::prelude::*;

fn main() {
	let apt = Apt::new().unwrap();
	let hid = Hid::new().unwrap();
	let gfx = Gfx::new().unwrap();
	let _console = Console::new(gfx.bottom_screen.borrow_mut());

	// let c2d = Instance::new().unwrap();

	let mut c3d_tex = C3D_Tex {
		__bindgen_anon_1: C3D_Tex__bindgen_ty_1 {
			data: std::ptr::null_mut(),
		},
		_bitfield_align_1: [],
		_bitfield_1: citro2d_sys::__BindgenBitfieldUnit::<[u8; 4]>::new([0; _]),
		__bindgen_anon_2: C3D_Tex__bindgen_ty_2 { dim: 0 },
		param: 0,
		border: 0,
		__bindgen_anon_3: C3D_Tex__bindgen_ty_3 { lodParam: 0 },
	};
	let mut tex3ds_subtexture = Tex3DS_SubTexture {
		width: 10,
		height: 10,

		// What is the coordinate space for these?
		left: 10f32,
		top: 10f32,
		right: 20f32,
		bottom: 20f32,
	};
	let c2d_image = C2D_Image {
		tex: &raw mut c3d_tex,
		subtex: &raw mut tex3ds_subtexture,
	};
	let c2d_drawparams = C2D_DrawParams {
		pos: C2D_DrawParams__bindgen_ty_1 {
			x: 0.,
			y: 0.,
			w: 0.,
			h: 0.,
		},
		center: C2D_DrawParams__bindgen_ty_2 { x: 0., y: 0. },
		depth: 1.,
		angle: 0.,
	};
	let c2d_sprite = C2D_Sprite {
		image: c2d_image,
		params: c2d_drawparams,
	};

	println!("Hi");

	while apt.main_loop() {}
}
