use anyhow::Ok;
use fastembed::{
    EmbeddingModel, ImageEmbedding, ImageEmbeddingModel, ImageInitOptions, InitOptions,
    InitOptionsUserDefined, TextEmbedding, TokenizerFiles, UserDefinedEmbeddingModel,
};
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
use tch::Device;

use crate::utilities::errors::AppError;

#[derive(Clone. Copy, Debug)]
pub struct AI {
    pub text_model: TextEmbedding,
    pub image_model: ImageEmbeddingModel,
}

impl AI {
    pub fn new() -> Result<Self, AppError> {
        let device = Device::cuda_if_available();

        // Model for text embeddings
        let text_model = build_fastembed_text_embedding()?;

        // Multi-modal model for both image and text query embeddings
        let image_model = build_fastembed_image_embedding()?;

        Ok(Self {
            text_model,
            image_model,
        })
    }

    // Generate embedding for text
    pub fn embed_text(&self, text: &str) -> Result<Vec<f32>, AppError> {
        let embedding = self.text_model.encode(&[text]);
        Ok(embedding.into_iter().next().unwrap())
    }

    // Generate embedding for an image from its bytes
    pub fn embed_image(&self, image_bytes: &[u8]) -> Result<Vec<f32>, AppError> {
        let image = image::load_from_memory(image_bytes)?.to_rgb8();
        let embeddings = self.image_model.encode_image(&[image])?;
        Ok(embeddings.into_iter().next().unwrap())
    }

    // Generate embedding for the user's search query (using the same model as images)
    pub fn embed_query(&self, query: &str) -> Result<Vec<f32>, AppError> {
        let embeddings = self.image_model.encode_text(&[query])?;
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
        .create_model()?;

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

    let mut model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
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

    let mut model = ImageEmbedding::try_new(
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
