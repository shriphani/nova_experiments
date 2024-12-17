use ff::{PrimeField, PrimeFieldBits};
use neptune::{circuit::poseidon_hash, poseidon::PoseidonConstants};
use nova_snark::{frontend::ConstraintSystem, traits::circuit::StepCircuit};

#[derive(Debug, Clone)]
pub struct PreImageCircuit<Scalar: PrimeField> {
    pub pre_image: Vec<Scalar>,
}

impl<Scalar: PrimeField + PrimeFieldBits> StepCircuit<Scalar> for PreImageCircuit<Scalar> {
    fn arity(&self) -> usize {
        1
    }

    fn synthesize<CS: nova_snark::frontend::ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        z: &[nova_snark::frontend::num::AllocatedNum<Scalar>],
    ) -> Result<
        Vec<nova_snark::frontend::num::AllocatedNum<Scalar>>,
        nova_snark::frontend::SynthesisError,
    > {
        let constants = PoseidonConstants::new_with_strength(neptune::Strength::Standard);

        poseidon_hash(&mut cs, self.pre_image, &constants);
    }
}
