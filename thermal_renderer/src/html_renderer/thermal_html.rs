use crate::html_renderer::HtmlRow;
use base64::engine::general_purpose;
use base64::Engine;
use png::{ColorType, Encoder};
use thermal_parser::graphics::{Image, VectorGraphic};
use thermal_parser::text::TextSpan;

pub fn encode_html_image(image: &Image) -> HtmlRow {
    // Create a buffer to hold the PNG image data
    let mut png_data: Vec<u8> = Vec::new();

    let mut encoder = Encoder::new(&mut png_data, image.w, image.h);
    encoder.set_color(ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().expect("Failed to write PNG header");
    writer
        .write_image_data(&image.as_grayscale())
        .expect("Failed to write PNG image data");

    writer.finish().expect("Error encoding png");

    let base64_encoded_image = general_purpose::STANDARD_NO_PAD.encode(&png_data);

    HtmlRow {
        y: image.y,
        height: image.h,
        content: format!(
            "<img style='left: {}px;' class='img' width='{}' src='data:image/png;base64, {}' />",
            image.x, image.w, base64_encoded_image
        ),
    }
}

pub fn graphics_to_svg(graphics: &Vec<VectorGraphic>) -> HtmlRow {
    if graphics.is_empty() {
        return HtmlRow::empty();
    }

    let mut svg: Vec<String> = vec![];
    let mut min_x = u32::MAX;
    let mut min_y = u32::MAX;
    let mut width = 0;
    let mut height = 0;

    for graphic in graphics {
        match graphic {
            VectorGraphic::Rectangle(rectangle) => {
                min_x = min_x.min(rectangle.x);
                min_y = min_y.min(rectangle.y);
            }
        }
    }

    for graphic in graphics {
        match graphic {
            VectorGraphic::Rectangle(rectangle) => {
                let x = rectangle.x.saturating_sub(min_x);
                let y = rectangle.y.saturating_sub(min_y);
                width = width.max(x + rectangle.w);
                height = height.max(y + rectangle.h);

                svg.push(format!(
                    "<rect width='{}' height='{}' x='{}' y='{}' fill='black' />",
                    rectangle.w, rectangle.h, x, y
                ));
            }
        }
    }

    HtmlRow {
        y: min_y,
        height,
        content: format!(
            "<svg style='left: {}px;' class='gfx' width='{}' height='{}'>{}</svg>",
            min_x,
            width,
            height,
            svg.join("\n")
        ),
    }
}

//                                             y,   height, content
pub fn spans_to_html(
    spans: &Vec<TextSpan>,
    x_offset: u32,
    max_height: u32,
    baseline_ratio: f32,
) -> HtmlRow {
    if spans.is_empty() {
        return HtmlRow::empty();
    }
    let mut spans_html: Vec<String> = vec![];
    let mut height = 0;
    let mut min_y = u32::MAX;

    //TODO get the min x and offset the top by it
    //This way the spans are relatively aligned to the top
    //of the html row element

    for span in spans {
        height = height.max(span.character_height);
        let (y, content) = span_to_html(span, x_offset, max_height, baseline_ratio);
        min_y = min_y.min(y);
        spans_html.push(content);
    }

    HtmlRow {
        y: min_y,
        height,
        content: spans_html.join("\n"),
    }
}

const STRETCH_W_CLASSES: [&str; 7] = ["w2", "w3", "w4", "w5", "w6", "w7", "w8"];
const STRETCH_H_CLASSES: [&str; 7] = ["h2", "h3", "h4", "h5", "h6", "h7", "h8"];

fn span_to_html(
    text: &TextSpan,
    x_offset: u32,
    max_height: u32,
    baseline_ratio: f32,
) -> (u32, String) {
    //All of this is to calculate the offset for smaller characters
    //When a larger character is in the same line.
    //We need to adjust the y to try and match the character baselines
    let max_height_baseline = max_height as f32 * baseline_ratio;
    let span_baseline = text.character_height as f32 * baseline_ratio;
    let baseline_offset = (max_height_baseline - span_baseline) as u32;

    let mut class_list = vec![];

    let (x, y) = match &text.dimensions {
        None => (0, 0),
        Some(d) => (d.x, d.y),
    };

    if text.bold {
        class_list.push("b");
    }

    if text.italic {
        class_list.push("i");
    }

    if text.strikethrough > 0 {
        class_list.push("s");
    }

    if text.strikethrough > 1 {
        class_list.push("sd");
    }

    let str_w = match text.stretch_width as u32 {
        2 => STRETCH_W_CLASSES[0],
        3 => STRETCH_W_CLASSES[1],
        4 => STRETCH_W_CLASSES[2],
        5 => STRETCH_W_CLASSES[3],
        6 => STRETCH_W_CLASSES[4],
        7 => STRETCH_W_CLASSES[5],
        8 => STRETCH_W_CLASSES[6],
        _ => "",
    };

    if !str_w.is_empty() {
        class_list.push(str_w)
    }

    let str_h = match text.stretch_height as u32 {
        2 => STRETCH_H_CLASSES[0],
        3 => STRETCH_H_CLASSES[1],
        4 => STRETCH_H_CLASSES[2],
        5 => STRETCH_H_CLASSES[3],
        6 => STRETCH_H_CLASSES[4],
        7 => STRETCH_H_CLASSES[5],
        8 => STRETCH_H_CLASSES[6],
        _ => "",
    };

    if !str_h.is_empty() {
        class_list.push(str_h);
    }

    if !str_w.is_empty() || !str_h.is_empty() {
        class_list.push("str");
    }

    if text.inverted {
        class_list.push("in");
    }

    if text.underline > 0 {
        class_list.push("u");
    }

    if text.underline > 1 {
        class_list.push("ud");
    }

    if text.upside_down == true {
        class_list.push("upd");
    }

    (
        y,
        format!(
            "<span style='left: {}px; top: {}px' class='{}'>{}</span>",
            x_offset + x,
            baseline_offset,
            class_list.join(" "),
            text.text
        ),
    )
}
