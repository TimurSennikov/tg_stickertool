pub mod converter {
    use opencv::{core, imgcodecs, imgproc, prelude::*};

    pub struct Converter {}

    impl Converter {
        pub fn split_for_tg(img_orig: &Mat) -> Result<Vec<Mat>, &'static str> {
            let mut dst: Vec<Mat> = vec![];

            let mut img: Mat = Mat::default();
            imgproc::resize(
                img_orig,
                &mut img,
                core::Size::new(512 * 5, 512 * 5),
                0.0,
                0.0,
                imgproc::INTER_LINEAR,
            )
            .unwrap();

            let w = img.rows();
            let h = img.cols();

            let stepx = w / 5;
            let stepy = h / 5;

            for j in 1..6 {
                for i in 1..6 {
                    let xmin = (stepx * i) - (w / 5);
                    let ymin = (stepy * j) - (h / 5);

                    dst.push(
                        Mat::roi(
                            &img,
                            core::Rect {
                                x: xmin,
                                y: ymin,
                                width: (w / 5),
                                height: (h / 5),
                            },
                        )
                        .unwrap()
                        .try_clone()
                        .unwrap(),
                    );
                }
            }

            println!("Final number of generated photos: {}", dst.len());

            Ok(dst)
        }

        pub fn crop_for_tg(img: &Mat) -> Result<Mat, &'static str> {
            let mut dst: Mat = Mat::default();

            imgproc::resize(
                img,
                &mut dst,
                core::Size::new(512, 512),
                0.0,
                0.0,
                imgproc::INTER_LINEAR,
            )
            .unwrap();

            Ok(dst)
        }

        pub fn prepare_for_tg(img: &(impl MatTrait + core::ToInputArray)) -> bytes::Bytes {
            let mut buf: core::Vector<u8> = core::Vector::new();
            imgcodecs::imencode(".png", img, &mut buf, &core::Vector::from(vec![])).unwrap();

            bytes::Bytes::from(buf.to_vec())
        }
    }
}
