use anyhow::anyhow;
use bytes::Bytes;
use candid::CandidType;
use prost::Message;
use serde::Deserialize;
use std::cell::RefCell;
use tract_ndarray::s;
use tract_onnx::prelude::*;

// The maximum distance between face embeddings of the same person.
const THRESHOLD: f32 = 0.85;

type Model = SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

thread_local! {
    static FACE_DETECTION: RefCell<Option<Model>> = RefCell::new(None);
    static FACE_RECOGNITION: RefCell<Option<Model>> = RefCell::new(None);
    static DB: RefCell<Vec<(String, Embedding)>> = RefCell::new(vec![]);
}

#[derive(CandidType, Deserialize, Clone)]
pub struct BoundingBox {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl BoundingBox {
    fn new(raw: &[f32]) -> Self {
        Self {
            left: raw[0],
            top: raw[1],
            right: raw[2],
            bottom: raw[3],
        }
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Embedding {
    v0: Vec<f32>,
}

impl Embedding {
    fn distance(&self, other: &Self) -> f32 {
        let result: f32 = self
            .v0
            .iter()
            .zip(other.v0.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum();
        result.sqrt()
    }
}

#[derive(CandidType, Deserialize)]
pub struct Person {
    label: String,
    score: f32,
}

fn setup_facedetect(bytes: Bytes) -> TractResult<()> {
    let proto: tract_onnx::pb::ModelProto = tract_onnx::pb::ModelProto::decode(bytes)?;
    let ultraface = tract_onnx::onnx()
        .model_for_proto_model(&proto)?
        .into_optimized()?
        .into_runnable()?;
    FACE_DETECTION.with_borrow_mut(|m| {
        *m = Some(ultraface);
    });
    Ok(())
}

fn setup_facerec(bytes: Bytes) -> TractResult<()> {
    let proto: tract_onnx::pb::ModelProto = tract_onnx::pb::ModelProto::decode(bytes)?;
    let facerec = tract_onnx::onnx()
        .model_for_proto_model(&proto)?
        .into_optimized()?
        .into_runnable()?;
    FACE_RECOGNITION.with_borrow_mut(|m| {
        *m = Some(facerec);
    });
    Ok(())
}

pub fn setup(facedetect: Bytes, facerec: Bytes) -> TractResult<()> {
    setup_facedetect(facedetect)?;
    setup_facerec(facerec)
}

/// Returns a bounding box around the face detected in the given image.
pub fn detect(image: Vec<u8>) -> Result<(BoundingBox, f32), anyhow::Error> {
    FACE_DETECTION.with_borrow(|model| {
        let model = model.as_ref().unwrap();
        let image = image::load_from_memory(&image)?.to_rgb8();

        // The model accepts an image of size 320x240px.
        let image =
            image::imageops::resize(&image, 320, 240, ::image::imageops::FilterType::Triangle);

        const MEAN: [f32; 3] = [0.485, 0.456, 0.406];
        const STD: [f32; 3] = [0.229, 0.224, 0.225];
        let tensor = tract_ndarray::Array4::from_shape_fn((1, 3, 240, 320), |(_, c, y, x)| {
            (image[(x as u32, y as u32)][c] as f32 / 255.0 - MEAN[c]) / STD[c]
        });

        let result = model.run(tvec!(Tensor::from(tensor).into()))?;

        let confidences = result[0]
            .to_array_view::<f32>()?
            .slice(s![0, .., 1])
            .to_vec();

        let boxes: Vec<_> = result[1].to_array_view::<f32>()?.iter().cloned().collect();
        let boxes: Vec<_> = boxes.chunks(4).map(BoundingBox::new).collect();
        let boxes: Vec<_> = boxes.iter().zip(confidences.iter()).collect();

        let best = boxes
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .ok_or(anyhow!("No face detected"))?;

        let best = (best.0.clone(), best.1.clone());
        Ok(best)
    })
}

/// Computes a face embedding corresponding to the given image of a face.
pub fn embedding(image: Vec<u8>) -> Result<Embedding, anyhow::Error> {
    FACE_RECOGNITION.with_borrow(|model| {
        let model = model.as_ref().unwrap();
        let image = image::load_from_memory(&image)?.to_rgb8();

        // The model accepts an image of size 160x160px.
        let image =
            image::imageops::resize(&image, 160, 160, ::image::imageops::FilterType::Triangle);

        let tensor = tract_ndarray::Array4::from_shape_fn((1, 3, 160, 160), |(_, c, y, x)| {
            image[(x as u32, y as u32)][c] as f32 / 255.0
        });

        let result = model.run(tvec!(Tensor::from(tensor).into()))?;

        let v0 = result[0]
            .to_array_view::<f32>()?
            .into_iter()
            .cloned()
            .collect();

        Ok(Embedding { v0 })
    })
}

/// Returns the person whose face embedding is the closest to the face embedding
/// of the given image.
pub fn recognize(image: Vec<u8>) -> Result<Person, anyhow::Error> {
    let emb = embedding(image)?;
    DB.with_borrow(|db| {
        let emb = &emb;
        let best = db
            .iter()
            .min_by(|a, b| f32::partial_cmp(&a.1.distance(emb), &b.1.distance(emb)).unwrap());
        let best = best.ok_or(anyhow!("Unknown person"))?.clone();
        let label = best.0;
        let score = best.1.distance(emb);
        if score > THRESHOLD {
            return Err(anyhow!("Unknown person"));
        }
        Ok(Person { label, score })
    })
}

/// Records a new person with the given name and face image into the state.
pub fn add(label: String, image: Vec<u8>) -> Result<Embedding, anyhow::Error> {
    let emb = embedding(image)?;
    DB.with_borrow_mut(|db| {
        db.push((label, emb.clone()));
    });
    Ok(emb)
}
