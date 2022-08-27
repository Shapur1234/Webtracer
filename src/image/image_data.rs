use std::ops::Index;

use serde::{ser::SerializeTuple, Deserialize, Deserializer, Serialize, Serializer};

use super::{get_const_image, ImageID};
use crate::draw::Color;
use crate::vector::Vec2D;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageData {
    #[serde(deserialize_with = "data_from_file")]
    #[serde(serialize_with = "data_to_file")]
    pub data: (Vec<Color>, ImageID),
    pub size: Vec2D<u32>,
}

fn data_from_file<'de, D>(deserializer: D) -> Result<(Vec<Color>, ImageID), D::Error>
where
    D: Deserializer<'de>,
{
    let images: (Vec<Color>, ImageID) = Deserialize::deserialize(deserializer)?;
    Ok((get_const_image(images.1.clone()).data.0.clone(), images.1).clone())
}

fn data_to_file<S>(x: &(Vec<Color>, ImageID), s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut tup = s.serialize_tuple(2)?;
    tup.serialize_element(&Vec::<Color>::new())?;
    tup.serialize_element(&x.1)?;
    tup.end()
}

impl ImageData {
    pub(super) fn new(image_id: ImageID, width: u32, data: Vec<u8>) -> ImageData {
        let out_len = (data.len() / 3) as u32;
        assert!(out_len % width == 0, "Data length and width don't match up");

        ImageData {
            data: (
                (0..(out_len as usize))
                    .into_iter()
                    .map(|x| Color::new(data[x * 3], data[x * 3 + 1], data[x * 3 + 2]))
                    .collect(),
                image_id,
            ),
            size: Vec2D::new(
                width,
                ((data.len() / 3) / (width as usize)).try_into().unwrap(),
            ),
        }
    }
}

impl Index<Vec2D<u32>> for ImageData {
    type Output = Color;

    fn index(&self, index: Vec2D<u32>) -> &Self::Output {
        &self.data.0[(index.y * self.size.x + index.x) as usize]
    }
}

impl Index<Vec2D<f32>> for ImageData {
    type Output = Color;

    fn index(&self, index: Vec2D<f32>) -> &Self::Output {
        let index_clamped = index.clamp(0.0, 1.0);
        let mut index_u32 = Vec2D::new(
            (index_clamped.x * (self.size.x as f32)) as u32,
            (index_clamped.y * (self.size.y as f32)) as u32,
        );

        if index_u32.x >= self.size.x {
            index_u32.x = self.size.x - 1;
        }
        if index_u32.y >= self.size.y {
            index_u32.y = self.size.y - 1
        }

        &self[index_u32]
    }
}
