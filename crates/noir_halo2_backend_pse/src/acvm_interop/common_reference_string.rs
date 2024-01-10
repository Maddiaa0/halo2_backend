use crate::{
    circuit_translator::NoirHalo2Translator, halo2_params::constuct_halo2_params_from_aztec_crs,
    PseHalo2,
};
use acvm::{
    acir::{circuit::Circuit, native_types::WitnessMap},
};
use noir_halo2_backend_common::errors::BackendError;
use pse_halo2wrong::curves::bn256::Fr;
use std::marker::PhantomData;

trait CommonReferenceString {
    type Error;

    /// Generate common reference string from
    /// Aztec CRS ceremony
    async fn generate_common_reference_string(
        &self,
        circuit: &Circuit,
    ) -> Result<Vec<u8>, Self::Error>;

    /// Update common reference string from
    /// Aztec CRS ceremony
    async fn update_common_reference_string(
        &self,
        common_reference_string: Vec<u8>,
        circuit: &Circuit,
    ) -> Result<Vec<u8>, Self::Error>;
}

impl CommonReferenceString for PseHalo2 {
    type Error = BackendError;

    /// Generate common reference string from
    /// Aztec CRS ceremony
    async fn generate_common_reference_string(
        &self,
        circuit: &Circuit,
    ) -> Result<Vec<u8>, Self::Error> {
        let mut common_reference_string = Vec::new();

        let translator = NoirHalo2Translator::<Fr> {
            circuit: circuit.clone(),
            witness_values: WitnessMap::new(),
            _marker: PhantomData::<Fr>,
        };
        constuct_halo2_params_from_aztec_crs(translator)
            .await?
            .write_custom(
                &mut common_reference_string,
                pse_halo2wrong::halo2::SerdeFormat::RawBytes,
            )
            .unwrap();
        Ok(common_reference_string)
    }

    /// Update common reference string from
    /// Aztec CRS ceremony
    async fn update_common_reference_string(
        &self,
        _common_reference_string: Vec<u8>,
        circuit: &Circuit,
    ) -> Result<Vec<u8>, Self::Error> {
        let mut common_reference_string = Vec::new();

        let translator = NoirHalo2Translator::<Fr> {
            circuit: circuit.clone(),
            witness_values: WitnessMap::new(),
            _marker: PhantomData::<Fr>,
        };
        constuct_halo2_params_from_aztec_crs(translator)
            .await?
            .write_custom(
                &mut common_reference_string,
                pse_halo2wrong::halo2::SerdeFormat::RawBytes,
            )
            .unwrap();

        Ok(common_reference_string)
    }
}
