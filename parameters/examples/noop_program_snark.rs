use snarkos_dpc::base_dpc::{instantiated::Components, parameters::SystemParameters, BaseDPCComponents, DPC};
use snarkos_errors::dpc::DPCError;
use snarkos_models::algorithms::SNARK;
use snarkos_utilities::{bytes::ToBytes, to_bytes};

use rand::thread_rng;
use std::path::PathBuf;

mod utils;
use utils::store;

pub fn setup<C: BaseDPCComponents>() -> Result<(Vec<u8>, Vec<u8>), DPCError> {
    let rng = &mut thread_rng();
    let system_parameters = SystemParameters::<C>::load()?;

    let noop_program_snark_parameters = DPC::<C>::generate_noop_program_snark_parameters(&system_parameters, rng)?;
    let noop_program_snark_pk = to_bytes![noop_program_snark_parameters.proving_key]?;
    let noop_program_snark_vk: <C::NoopProgramSNARK as SNARK>::VerificationParameters =
        noop_program_snark_parameters.verification_key.into();
    let noop_program_snark_vk = to_bytes![noop_program_snark_vk]?;

    println!("noop_program_snark_pk.params\n\tsize - {}", noop_program_snark_pk.len());
    println!("noop_program_snark_vk.params\n\tsize - {}", noop_program_snark_vk.len());
    Ok((noop_program_snark_pk, noop_program_snark_vk))
}

pub fn main() {
    let (program_snark_pk, program_snark_vk) = setup::<Components>().unwrap();
    store(
        &PathBuf::from("noop_program_snark_pk.params"),
        &PathBuf::from("noop_program_snark_pk.checksum"),
        &program_snark_pk,
    )
    .unwrap();
    store(
        &PathBuf::from("noop_program_snark_vk.params"),
        &PathBuf::from("noop_program_snark_vk.checksum"),
        &program_snark_vk,
    )
    .unwrap();
}