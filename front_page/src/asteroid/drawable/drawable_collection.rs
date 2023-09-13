use super::*;

pub trait DrawableCollection<T: Drawable> {
    fn get_lst(&self) -> &Vec<T>;
}

impl<T: Drawable> Drawable for dyn DrawableCollection<T> {
    fn draw(
        &self,
        context: &WebGlRenderingContext,
        offset: Point2<f64>,
        rotation: f64,
        scale: f64,
    ) -> Result<(), JsValue> {
        let lst = self.get_lst();
        for draw in lst.into_iter() {
            draw.draw(&context, offset, rotation, scale)?;
        }
        Ok(())
    }
}
