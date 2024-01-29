use std::{collections::HashSet, io, path::Path};
use word2vec::wordvectors::WordVector;

use thiserror::Error;

pub struct Model {
    model: WordVector,
    vocabulary: HashSet<String>,
}

impl Model {
    pub fn from_file<P: AsRef<Path>>(model_file: P) -> Result<Self, Error> {
        // let vocab = fs::read_to_string(vocab_file)?.lines().map(str::to_string).collect::<Vec<String>>();
        // let vocab_set = vocab.iter().map(String::to_string).collect::<HashSet<String>>();

        // let model = hdf5::File::open(model_file)?;
        // let dataset = &model.datasets()?[0];

        // let reader = dataset.as_reader().obj;
        
        let model = WordVector::load_from_binary(&model_file.as_ref().to_string_lossy())?;
        let vocab = model.get_words().collect();

        Ok(Self {
            model, vocabulary: vocab
        })
    }

    pub fn vec_for(&self, string: &str) -> Result<WordVec, Error> {
        if !self.vocabulary.contains(string) {
            Err(Error::NotInVocabulary(string.to_owned()))
        } else {
            Ok(WordVec { vec: self.model.get_vector(string).unwrap().clone() })
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("word `{0}` not in vocabulary")]
    NotInVocabulary(String),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("hdf error: {0}")]
    Word2Vec(#[from] word2vec::errors::Word2VecError)
}

#[derive(Debug, Clone)]
pub struct WordVec {
    vec: Vec<f32>,
}

impl WordVec {
    pub fn into_inner(self) -> Vec<f32> {
        self.vec
    }
}
