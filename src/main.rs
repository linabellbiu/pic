mod pb;

// 解析出来的图片处理参数
struct ImageSpec {
    specs: Vec<Spec>,
}

// 每个参数的是我们支持的某种方式
enum Spec {
    Resize(Resize),
    // Crop(Crop),
}
    
// 处理的图片
struct Resize {
    width: u32,
    height: u32,
}


fn main() {
    println!("Hello, world!");
}
