use std::{collections::HashSet, io, path::Path};

use thiserror::Error;
use word2vec::wordvectors::WordVector;

pub struct Model {
    model: WordVector,
    vocabulary: HashSet<String>,
    vocab_vec: Vec<String>,
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
        let vocab_vec = model.get_words().collect();

        Ok(Self {
            model,
            vocabulary: vocab,
            vocab_vec,
        })
    }

    pub fn vec_for(&self, string: &str) -> Result<WordVec, Error> {
        if !self.vocabulary.contains(string) {
            Err(Error::NotInVocabulary(string.to_owned()))
        } else {
            let vec = self.model.get_vector(string).unwrap().clone();
            Ok(WordVec { vec })
        }
    }

    pub fn random_word(&self) -> &str {
        loop {
            let word = &self.vocab_vec[fastrand::usize(..self.vocab_vec.len())];
            let valid = word
                .chars()
                .all(|c| ('a'..='z').contains(&c) || ['ä', 'ö', 'ü', 'ß', '-'].contains(&c));
            if valid {
                break word;
            }
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
    Word2Vec(#[from] word2vec::errors::Word2VecError),
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
