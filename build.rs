use image::ImageEncoder;

fn main() {
    // 只有 Windows 平台需要设置图标和子系统
    if cfg!(target_os = "windows") {
        // 读取 icon.png，生成 ico 数据并嵌入
        let png_path = std::path::Path::new("src/assets/icon.png");
        if png_path.exists() {
            let img = image::open(png_path).expect("无法读取 icon.png");
            // 生成 32x32 的 ICO
            let rgba = img.to_rgba8();
            let resized = image::imageops::resize(&rgba, 32, 32, image::imageops::Lanczos3);
            let mut ico_data = Vec::new();
            {
                let encoder = image::codecs::ico::IcoEncoder::new(&mut ico_data);
                encoder
                    .write_image(
                        resized.as_raw(),
                        resized.width(),
                        resized.height(),
                        image::ExtendedColorType::Rgba8,
                    )
                    .expect("编码 ico 失败");
            }
            // 将 ico 数据写入临时文件
            let out_dir = std::env::var("OUT_DIR").unwrap();
            let ico_path = std::path::Path::new(&out_dir).join("app_icon.ico");
            std::fs::write(&ico_path, &ico_data).expect("写入 ico 文件失败");
            // 通过 winres 嵌入图标并设置 Windows 子系统（隐藏控制台窗口）
            let mut res = winres::WindowsResource::new();
            res.set_icon(&ico_path.to_string_lossy());
            res.compile().expect("编译资源文件失败");
        } else {
            println!("cargo:warning=icon.png 未找到，将使用默认图标");
        }
    }
}
