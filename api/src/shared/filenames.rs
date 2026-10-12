use rand::distr;

pub fn generate_random_filename(content_type: &mime::Mime) -> String {
    use rand::RngExt as _;

    let length = 10;
    let filename: String = rand::rng()
        .sample_iter(&distr::Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    format!("{}.{}", filename, extension(content_type))
}

fn extension(content_type: &mime::Mime) -> &str {
    if content_type == &mime::IMAGE_BMP {
        return "bmp";
    }
    if content_type == &mime::IMAGE_JPEG {
        return "jpeg";
    }
    if content_type == &mime::IMAGE_PNG {
        return "png";
    }
    if content_type == &mime::IMAGE_GIF {
        return "gif";
    }

    return "bin";
}
