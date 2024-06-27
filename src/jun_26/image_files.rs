// // use image::png::PNGEncoder;
// // use image::ColorType;
// // use std::fs::File;
// // /// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the /// file named `filename`.
// // fn write_image(
// //     filename: &str,
// //     pixels: &[u8],
// //     bounds: (usize, usize),
// // ) -> Result<(), std::io::Error> {
// //     let output = File::create(filename)?;   // '?' raise error
// //     // match  File::create(filename){
// //     //     Ok(_) => todo!(),
// //     //     Err(_) => todo!(),
// //     // }

// //     // if let Err(err) = File::create(filename) {
// //     //     println!("{:?}",err);
// //     // }
// //     let encoder = PNGEncoder::new(output);
// //     encoder.encode(
// //         &pixels,
// //         bounds.0 as u32,
// //         bounds.1 as u32,
// //         ColorType::Gray(8),
// //     )?;
// //     Ok(())
// // }

// #[test]
// fn multi_thread_make_file() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 5 {
//         eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
//         eprintln!(
//             "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
//             args[0]
//         );
//         std::process::exit(1);
//     }
//     let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
//     let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
//     let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
//     let mut pixels = vec![0; bounds.0 * bounds.1];
//     render(&mut pixels, bounds, upper_left, lower_right);
//     write_image(&args[1], &pixels, bounds).expect("error writing PNG file");

//     let threads = 8;
//     let rows_per_band = bounds.1 / threads + 1;
//     {
//         let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
//         crossbeam::scope(|spawner| {
//             // call closure, parameter is spawner
//             for (i, band) in bands.into_iter().enumerate() {
//                 let top = rows_per_band * i;
//                 let height = band.len() / bounds.0;
//                 let band_bounds = (bounds.0, height);
//                 let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
//                 let band_lower_right =
//                     pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
//                 spawner.spawn(move |_| {
//                     render(band, band_bounds, band_upper_left, band_lower_right);
//                 });
//             }
//         })
//         .unwrap();
//     }
// }

// #[test]
// fn handling_err() {
//     let a = vec![1; 3]; // [1, 1, 1]
//     println!("{:?}", a);
// }
