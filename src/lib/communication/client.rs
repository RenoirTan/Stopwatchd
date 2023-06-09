//! Messages passed from clients to `swd` server.

use serde::{Serialize, Deserialize};

use crate::util::iter_into_vec;

pub use super::request_specifics::SpecificArgs;

/// Common arguments for requests.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommonArgs {
    /// List of stopwatches the specified action should be applied to.
    pub raw_identifiers: Vec<String>,
    /// Whether to return verbose/more detailed information.
    pub verbose: bool
}

impl CommonArgs {
    /// Create a new [`CommonArgs`] object.
    pub fn new(identifiers: Vec<String>, verbose: bool) -> Self {
        Self { raw_identifiers: identifiers, verbose }
    }

    /// Create a new [`CommonArgs`] object from an [`Iterator`] of
    /// [`Identifier`]s.
    /// 
    /// # Example
    /// 
    /// ```
    /// // `&str` implements `Into<Identifier>`
    /// let identifiers = ["sw1", "sw2"];
    /// let verbose = false;
    /// let common = CommonArgs::from_iter(identifiers, verbose);
    /// ```
    pub fn from_iter<I, T>(identifiers: I, verbose: bool) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>
    {
        Self::new(iter_into_vec(identifiers), verbose)
    }
}

impl Default for CommonArgs {
    fn default() -> Self {
        Self { raw_identifiers: vec![], verbose: false }
    }
}

/// A request from a client.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    /// Common arguments (not specific to one or a few actions).
    pub common_args: CommonArgs,
    /// Type of request and their arguments.
    pub specific_args: SpecificArgs
}

impl Request {
    /// Create a new [`Request`].
    pub fn new(common_args: CommonArgs, specific_args: SpecificArgs) -> Self {
        Self { common_args, specific_args }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        communication::{
            client::{Request, CommonArgs},
            request_specifics::StartArgs
        },
        traits::Codecable
    };

    #[test]
    fn test_cycle_0() {
        let specific = StartArgs { fix_bad_names: false }.into();
        let common = CommonArgs::from_iter([""], false);
        let request = Request::new(common, specific);

        let encoded = request.to_bytes().unwrap();
        let decoded = Request::from_bytes(&encoded).unwrap();

        assert_eq!(request, decoded);
    }

    #[test]
    fn test_cycle_1() {
        let specific = StartArgs { fix_bad_names: true }.into();
        let common = CommonArgs::from_iter(["random"], false);
        let request = Request::new(common, specific);

        let encoded = request.to_bytes().unwrap();
        let decoded = Request::from_bytes(&encoded).unwrap();

        assert_eq!(request, decoded);
    }
}