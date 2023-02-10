pub trait WindowTrait<'a> {
    type Event;

    fn size(&self) -> (u32, u32);
    fn set_title(&mut self, title: &str);
    fn running(&self) -> bool;
    fn stop(&mut self);

    fn events(&'a mut self) -> Vec<Self::Event>;
    fn clear(&mut self, r: f32, g: f32, b: f32);
    fn present(&mut self);
}
