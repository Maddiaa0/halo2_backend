use std::marker::PhantomData;

use acvm::acir::{circuit::Circuit, native_types::WitnessMap};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Halo2Backend {
    Prove(Prove),
    Verify,
    Version,
    VkAsFields,
    PkAsFields,
    WriteVk,
    WritePk,
    Contract,
    Gates,
    Info
}


#[derive(Parser, Debug)]
struct Prove {
    #[arg(short, long)]
    circuit: Option<String>,
    #[arg(short, long)]
    witness: Option<String>,
    #[arg(short, long)]
    vk: Option<String>,
    #[arg(short, long)]
    pk: Option<String>,
    #[arg(short, long)]
    output: Option<String>,
    // #[clap(short, long)]
    // proof: String,
    // #[clap(short, long)]
    // inputs: String,
    // #[clap(short, long)]
    // output: String,
    // #[clap(short, long)]
    // verbose: bool,

}


fn main() {
    let args = Halo2Backend::parse();


    match args {
        Halo2Backend::Prove(prove_cmd) => prove(prove_cmd),
        // Halo2Backend::Verify => verify(),
        // Halo2Backend::Version => version(),
        // Halo2Backend::VkAsFields => vk_as_fields(),
        // Halo2Backend::PkAsFields => pk_as_fields(),
        // Halo2Backend::WriteVk => write_vk(),
        // Halo2Backend::WritePk => write_pk(),
        // Halo2Backend::Contract => contract(),
        // Halo2Backend::Gates => gates(),
        // Halo2Backend::Info => info(),
        _ => todo!("lazy shite")
    }
}


// use halo2_plonk_api::{circuit_translator::NoirHalo2Translator, dimension_measure::DimensionMeasurement};
use noir_halo2_backend_pse::circuit_translator::NoirHalo2Translator;
use noir_halo2_backend_pse::dimension_measure::DimensionMeasurement;
    use pse_halo2wrong::{
        curves::bn256::Fr, halo2::dev::MockProver,
    };

fn prove(prove_cmd: Prove) {
    // Read the circuit file
    let circuit_path = prove_cmd.circuit.unwrap_or("./target/acir.gz".to_owned());
    let witness_path = prove_cmd.witness.unwrap_or("./target/witness.gz".to_owned());
    let _vk_path = prove_cmd.vk.unwrap_or("./target/vk.gz".to_owned());
    let _pk_path = prove_cmd.pk.unwrap_or("./target/pk.gz".to_owned());
    let _output_path = prove_cmd.output.unwrap_or("./target/proof.gz".to_owned());

    let circuit_bytes = std::fs::read(circuit_path).unwrap();
    let circuit = Circuit::read(&*circuit_bytes).expect("failed to read circuit");
    
    let witness_bytes = std::fs::read(witness_path).unwrap();
    let witness_values = WitnessMap::try_from(&*witness_bytes).expect("failed to read witness");

    let translator =
        NoirHalo2Translator::<Fr> { circuit, witness_values, _marker: PhantomData::<Fr> };
    let dimension = DimensionMeasurement::measure(&translator).unwrap();

    // TODO: is instance value the public inputs? does this need to change
    let instance = vec![Fr::from_raw([7u64, 0, 0, 0])];

    // run mock prover expecting success
    let prover = MockProver::run(dimension.k(), &translator, vec![instance]).unwrap();
    assert_eq!(prover.verify(), Ok(()));

}