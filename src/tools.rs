use crate::utils::max;
use image::ImageResult;
#[cfg(feature = "img")]
use image::{ImageBuffer, Rgb, RgbImage};
/// 给定数组绘制元素的条形统计图
/// - width指定宽度 height指定高度 name指定保存文件名
/// # Example
/// ```
/// use algori::tools::draw_bar_chart;
/// use algori::sorting::merge_sort;
/// let mut a = [0,9,2,3,0,8,2,4,9,6];
/// draw_bar_chart(&a,500,500,"排序前.png");
/// merge_sort(&mut a,|a,b|a<b);
/// draw_bar_chart(&a,500,500,"排序后.png");
/// ```
#[cfg(feature = "img")]
pub fn draw_bar_chart<T>(array: &[T], width: u32, height: u32, name: &str) -> ImageResult<()>
where
    T: Into<i32> + Copy + Ord,
{
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let background_color = Rgb([0, 0, 0]);

    for pixel in img.pixels_mut() {
        *pixel = background_color;
    }

    let max_element = array[max(&array)].into() as f32;

    let bar_color = Rgb([78, 232, 243]);

    let bar_width = width / array.len() as u32;

    for (i, &value) in array.iter().enumerate() {
        let bar_height = (value.into() as f32 / max_element * height as f32) as u32;
        let x_start = i as u32 * bar_width;
        let y_start = height - bar_height;

        for x in x_start..x_start + bar_width {
            for y in y_start..height {
                img.put_pixel(x, y, bar_color);
            }
        }
    }

    img.save(name)?;
    Ok(())
}
