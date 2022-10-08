use opencv::imgproc::rectangle;
use opencv::{
	highgui,
	prelude::*,
	Result,
	videoio,
	core,
};
use opencv::video::Tracker;

fn main() -> Result<()> {
    println!("Hello, world!");
	let window = "video capture";
	highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
	opencv::opencv_branch_32! {
		let mut cam = videoio::VideoCapture::new_default(0)?; // 0 is the default camera
	}
	opencv::not_opencv_branch_32! {
		let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
	}
	let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
		panic!("Unable to open default camera!");
	}


	let mut param: opencv::tracking::TrackerKCF_Params = opencv::tracking::TrackerKCF_Params::default().unwrap();

	let mut t = <dyn TrackerKCF>::create(param).unwrap();
	//let mut bounding_box = opencv::core::Rect::new(100, 100, 50, 50);
    let mut count: i32 = 0;
    let mut m = opencv::core::Mat::default();

    let mut frame = Mat::default();
    cam.read(&mut frame)?;
	let mut bounding_box = opencv::highgui::select_roi(&mut frame,true,true).unwrap();
//	println!("{0} {1}", bounding_box.x,bounding_box.y);

	t.init(&mut frame, bounding_box);

	println!("Start the tracking process, press ESC to quit.\n");
	loop {
		cam.read(&mut frame)?;
		if(frame.rows() == 0 || frame.cols() == 0)
		{
			break;			
		}
		t.update(&mut frame, &mut bounding_box);
//		println!("{0} {1}", bounding_box.x,bounding_box.y);
		

		rectangle(&mut frame, bounding_box,core::Scalar::new(0f64, -1f64, -1f64, -1f64), 2,8,0);

	    // show image with the tracked object
		highgui::imshow(window,&mut frame);

		//			highgui::imshow(window, &mut frame)?;
		let key = highgui::wait_key(10)?;
		if key > 0 && key != 255 {
			break;
		}
	}
	Ok(())
}