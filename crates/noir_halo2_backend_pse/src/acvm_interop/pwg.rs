use acvm::{acir::FieldElement, BlackBoxFunctionSolver, BlackBoxResolutionError};

use crate::PseHalo2;

impl BlackBoxFunctionSolver for PseHalo2 {
    fn schnorr_verify(
        &self,
        _public_key_x: &FieldElement,
        _public_key_y: &FieldElement,
        _signature: &[u8],
        _message: &[u8],
    ) -> Result<bool, BlackBoxResolutionError> {
        Err(BlackBoxResolutionError::Unsupported(acvm::acir::BlackBoxFunc::SchnorrVerify))
    }

    fn pedersen(
        &self,
        _inputs: &[FieldElement],
        _domain_separator: u32,
    ) -> Result<(FieldElement, FieldElement), BlackBoxResolutionError> {
        Err(BlackBoxResolutionError::Unsupported(acvm::acir::BlackBoxFunc::Pedersen))
    }

    fn fixed_base_scalar_mul(
        &self,
        _low: &FieldElement,
        _high: &FieldElement,
    ) -> Result<(FieldElement, FieldElement), BlackBoxResolutionError> {
        Err(BlackBoxResolutionError::Unsupported(acvm::acir::BlackBoxFunc::FixedBaseScalarMul))
    }
}
