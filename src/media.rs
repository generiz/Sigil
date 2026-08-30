#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    Image,
    Voice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaNormalization {
    DecodePixelsAndReencode,
    PcmToOpus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaPlanError {
    EmptyMedia,
    InvalidChunkSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkPlan {
    pub plaintext_len: usize,
    pub chunk_size: usize,
    pub chunk_count: usize,
}

impl ChunkPlan {
    pub fn new(plaintext_len: usize, chunk_size: usize) -> Result<Self, MediaPlanError> {
        if plaintext_len == 0 {
            return Err(MediaPlanError::EmptyMedia);
        }
        if chunk_size == 0 {
            return Err(MediaPlanError::InvalidChunkSize);
        }

        let chunk_count = 1 + (plaintext_len - 1) / chunk_size;
        Ok(Self {
            plaintext_len,
            chunk_size,
            chunk_count,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaTransferPlan {
    pub kind: MediaKind,
    pub normalization: MediaNormalization,
    pub chunks: ChunkPlan,
}

impl MediaTransferPlan {
    pub fn image(plaintext_len: usize, chunk_size: usize) -> Result<Self, MediaPlanError> {
        Ok(Self {
            kind: MediaKind::Image,
            normalization: MediaNormalization::DecodePixelsAndReencode,
            chunks: ChunkPlan::new(plaintext_len, chunk_size)?,
        })
    }

    pub fn voice(plaintext_len: usize, chunk_size: usize) -> Result<Self, MediaPlanError> {
        Ok(Self {
            kind: MediaKind::Voice,
            normalization: MediaNormalization::PcmToOpus,
            chunks: ChunkPlan::new(plaintext_len, chunk_size)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_count_rounds_up_without_overflow_formula() {
        let plan = ChunkPlan::new(130_000, 64_000).unwrap();
        assert_eq!(plan.chunk_count, 3);
    }

    #[test]
    fn image_plan_requires_pixel_reencode() {
        let plan = MediaTransferPlan::image(10_000, 4096).unwrap();
        assert_eq!(plan.kind, MediaKind::Image);
        assert_eq!(
            plan.normalization,
            MediaNormalization::DecodePixelsAndReencode
        );
    }

    #[test]
    fn voice_plan_requires_pcm_to_opus_normalization() {
        let plan = MediaTransferPlan::voice(10_000, 4096).unwrap();
        assert_eq!(plan.kind, MediaKind::Voice);
        assert_eq!(plan.normalization, MediaNormalization::PcmToOpus);
    }

    #[test]
    fn empty_media_and_zero_chunk_sizes_are_rejected() {
        assert_eq!(ChunkPlan::new(0, 4096), Err(MediaPlanError::EmptyMedia));
        assert_eq!(
            ChunkPlan::new(100, 0),
            Err(MediaPlanError::InvalidChunkSize)
        );
    }
}
