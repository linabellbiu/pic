// Engine trait：未来可以添加更多的 engine，主流程只需要替换 engine
pub trait Engine {
    // 对engine 按照specs:&[Spec]进行一系列有序的处理
    fn apply(&mut self, spec: &[Spec]);

    // 从engine 中生产目标图片,注意这里是用的是self
    fn generate(self, format: &ImageOutputFormat) -> Vec<u8>;
}

pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}
