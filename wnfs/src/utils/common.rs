use crate::error::FsError;
use anyhow::Result;
use std::fmt::Debug;
use wnfs_common::utils::error;

pub struct OnceCellDebug<T>(pub Option<T>);

impl<T: Debug> Debug for OnceCellDebug<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("OnceCell").field(&self.0).finish()
    }
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

pub(crate) fn split_last(path_segments: &[String]) -> Result<(&[String], &String)> {
    match path_segments.split_last() {
        Some((last, rest)) => Ok((rest, last)),
        None => error(FsError::InvalidPath),
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_last_splits_path_segments_into_tail_and_the_rest() {
        let path_segments = ["a".into(), "b".into(), "c".into()];
        let (rest, last) = split_last(&path_segments).unwrap();
        assert_eq!(rest, &["a", "b"]);
        assert_eq!(last, &"c");
    }
}
