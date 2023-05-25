use opencv::{
    core,
    imgcodecs,
    prelude::*,
    Result,
    videoio,
};

fn main() -> Result<()> {
    // 打开视频文件
    let filename: &str = "/tmp/video.mp4";
    let mut capture = videoio::VideoCapture::from_file(&filename, videoio::CAP_ANY)?;
    
    // 检查视频是否成功打开
    if !capture.is_opened()? {
        panic!("无法打开视频文件");
    }

    // 读取并显示视频帧
    let mut frame = Mat::default();
    let mut frame_id = 0;
    loop {
        capture.read(&mut frame)?;

        // 检查是否读取到了帧
        if frame.empty() {
            break;
        }

        let image_filename = format!("frame_{}.jpg", frame_id);
        imgcodecs::imwrite(&image_filename, &frame, &core::Vector::<i32>::new())?;
        println!("图像已保存为: {}", image_filename);

        frame_id += 1;
        if frame_id == 10 {
            break;
        }
    }
    
    Ok(())
}