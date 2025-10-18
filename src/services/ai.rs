use std::sync::{Arc, Mutex};

use ::image::DynamicImage;
use fastembed::{
    EmbeddingModel, ImageEmbedding, ImageEmbeddingModel, ImageInitOptions, InitOptions,
    TextEmbedding,
};
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
};
use tch::Device;

use crate::utilities::errors::AppError;

#[derive(Clone)]
pub struct AI {
    pub text_model: Arc<Mutex<TextEmbedding>>,
    pub image_model: Arc<Mutex<ImageEmbedding>>,
}

impl AI {
    pub fn new() -> Result<Self, AppError> {
        let text_model = build_fastembed_text_embedding()?;
        let image_model = build_fastembed_image_embedding()?;

        Ok(Self {
            text_model: Arc::new(Mutex::new(text_model)),
            image_model: Arc::new(Mutex::new(image_model)),
        })
    }

    // Generate embedding for text
    pub fn embed_text(&self, text: &str) -> Result<Vec<f32>, AppError> {
        let texts = vec![text];
        let mut model = self.text_model.lock().unwrap();
        let embeddings: Vec<Vec<f32>> = model
            .embed(texts, None)
            .map_err(|_| AppError::EmbeddingError)?;
        Ok(embeddings.into_iter().next().unwrap())
    }

    // Generate embedding for an image from its bytes
    pub fn embed_image_bytes(&self, image_bytes: &[u8]) -> Result<Vec<f32>, AppError> {
        let mut model = self.image_model.lock().unwrap();
        let embeddings: Vec<Vec<f32>> = model
            .embed_bytes(&[image_bytes], None)
            .map_err(|_| AppError::EmbeddingError)?;
        Ok(embeddings.into_iter().next().unwrap())
    }

    pub fn embed_dynamic_image(&self, img: DynamicImage) -> Result<Vec<f32>, AppError> {
        let mut model = self.image_model.lock().unwrap();
        let embeddings: Vec<Vec<f32>> = model
            .embed_images(vec![img])
            .map_err(|_| AppError::EmbeddingError)?;
        Ok(embeddings.into_iter().next().unwrap())
    }
}

pub async fn build_rust_bert_sentence_embedding() -> Result<SentenceEmbeddingsModel, AppError> {
    // SentenceEmbeddingsModelType
    //     DistiluseBaseMultilingualCased
    //     BertBaseNliMeanTokens
    //     AllMiniLmL12V2
    //     AllMiniLmL6V2
    //     AllDistilrobertaV1
    //     ParaphraseAlbertSmallV2
    //     SentenceT5Base
    let device = Device::cuda_if_available();
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .with_device(device)
        .create_model()
        .map_err(|e| AppError::SentenceEmbeddingsModelCreationError(e))?;

    // let sentences = ["this is an example sentence", "each sentence is converted"];
    // let output = model.encode(&sentences)?;

    Ok(model)
}

pub fn build_fastembed_text_embedding() -> Result<TextEmbedding, AppError> {
    // EmbeddingModel
    //     BAAI/bge-small-en-v1.5 - Default
    //     sentence-transformers/all-MiniLM-L6-v2
    //     mixedbread-ai/mxbai-embed-large-v1
    //     Qdrant/clip-ViT-B-32-text - pairs with clip-ViT-B-32-vision for image-to-text search
    //     BAAI/bge-large-en-v1.5
    //     BAAI/bge-small-zh-v1.5
    //     BAAI/bge-large-zh-v1.5
    //     BAAI/bge-base-en-v1.5
    //     sentence-transformers/all-MiniLM-L12-v2
    //     sentence-transformers/paraphrase-MiniLM-L12-v2
    //     sentence-transformers/paraphrase-multilingual-mpnet-base-v2
    //     lightonai/ModernBERT-embed-large
    //     nomic-ai/nomic-embed-text-v1
    //     nomic-ai/nomic-embed-text-v1.5 - pairs with nomic-embed-vision-v1.5 for image-to-text search
    //     intfloat/multilingual-e5-small
    //     intfloat/multilingual-e5-base
    //     intfloat/multilingual-e5-large
    //     Alibaba-NLP/gte-base-en-v1.5
    //     Alibaba-NLP/gte-large-en-v1.5

    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::ClipVitB32).with_show_download_progress(true),
    )
    .map_err(|_| AppError::TextEmbeddingCreationError)?;

    // let documents = vec![
    //     "passage: Hello, World!",
    //     "query: Hello, World!",
    //     "passage: This is an example passage.",
    //     // You can leave out the prefix but it's recommended
    //     "fastembed-rs is licensed under Apache  2.0",
    // ];

    // Generate embeddings with the default batch size, 256
    // let embeddings = model.embed(documents, None)?;

    // println!("Embeddings length: {}", embeddings.len()); // -> Embeddings length: 4
    // println!("Embedding dimension: {}", embeddings[0].len()); // -> Embedding dimension: 384

    Ok(model)
}

pub fn build_fastembed_image_embedding() -> Result<ImageEmbedding, AppError> {
    // ImageEmbeddingModel
    //     Qdrant/clip-ViT-B-32-vision - Default
    //     Qdrant/resnet50-onnx
    //     Qdrant/Unicom-ViT-B-16
    //     Qdrant/Unicom-ViT-B-32
    //     nomic-ai/nomic-embed-vision-v1.5

    let model = ImageEmbedding::try_new(
        ImageInitOptions::new(ImageEmbeddingModel::ClipVitB32).with_show_download_progress(true),
    )
    .map_err(|_| AppError::ImageEmbeddingCreationError)?;

    // let images = vec!["assets/image_0.png", "assets/image_1.png"];

    // Generate embeddings with the default batch size, 256
    // let embeddings = model.embed(images, None)?;

    // println!("Embeddings length: {}", embeddings.len()); // -> Embeddings length: 2
    // println!("Embedding dimension: {}", embeddings[0].len()); // -> Embedding dimension: 512

    Ok(model)
}
