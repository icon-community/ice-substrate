use frame_system;

/// AccountId of anything that implements frame_system::Config
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

/// Type that represent IconAddress
pub type IconAddress = sp_std::vec::Vec<u8>;

/// type that represnt the signed message
pub type SignedMessage = sp_std::vec::Vec<u8>;

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
/// type that represnt the error that can occur while validation the signature
pub enum SignatureValidationError {
	InvalidLength,
	InvalidFormat,
	InvalidMessage,
	MismatchedIconAddress,
	MismatchedIceAddress,
	Sha3Execution,
	ECRecoverExecution,
}
