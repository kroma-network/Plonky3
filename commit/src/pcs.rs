//! Traits for polynomial commitment schemes.

use alloc::vec;
use p3_field::Field;
use p3_matrix::dense::RowMajorMatrix;

use alloc::vec::Vec;
use p3_challenger::Challenger;

/// A (not necessarily hiding) polynomial commitment scheme, for committing to (batches of)
/// polynomials defined over the field `F`.
///
/// This high-level trait is agnostic with respect to the structure of a point; see `UnivariatePCS`
/// and `MultivariatePCS` for more specific subtraits.
// TODO: Should we have a super-trait for weakly-binding PCSs, like FRI outside unique decoding radius?
pub trait PCS<F: Field> {
    /// The commitment that's sent to the verifier.
    type Commitment;

    /// Data that the prover stores for committed polynomials, to help the prover with opening.
    type ProverData;

    /// The opening argument.
    type Proof;

    type Error;

    fn commit_batches(polynomials: Vec<RowMajorMatrix<F>>) -> (Self::Commitment, Self::ProverData);

    fn commit_batch(polynomials: RowMajorMatrix<F>) -> (Self::Commitment, Self::ProverData) {
        Self::commit_batches(vec![polynomials])
    }

    fn get_committed_value(prover_data: &Self::ProverData, poly: usize, value: usize) -> F;
}

pub trait UnivariatePCS<F: Field>: PCS<F> {
    fn open_multi_batches<FE, Chal>(
        prover_data: &[Self::ProverData],
        points: &[FE],
        challenger: &mut Chal,
    ) -> (Vec<Vec<Vec<FE>>>, Self::Proof)
    where
        FE: Field<Base = F>,
        Chal: Challenger<F>;

    fn verify_multi_batches<FE, Chal>(
        commits: &[Self::Commitment],
        points: &[FE],
        values: &[Vec<Vec<FE>>],
        proof: &Self::Proof,
    ) -> Result<(), Self::Error>
    where
        FE: Field<Base = F>,
        Chal: Challenger<F>;
}

pub trait MultivariatePCS<F: Field>: PCS<F> {
    fn open_multi_batches<FE, Chal>(
        prover_data: &[Self::ProverData],
        points: &[FE],
        challenger: &mut Chal,
    ) -> (Vec<Vec<Vec<FE>>>, Self::Proof)
    where
        FE: Field<Base = F>,
        Chal: Challenger<F>;

    fn verify_multi_batches<FE, Chal>(
        commits: &[Self::Commitment],
        points: &[FE],
        values: &[Vec<Vec<FE>>],
        proof: &Self::Proof,
    ) -> Result<(), Self::Error>
    where
        FE: Field<Base = F>,
        Chal: Challenger<F>;
}
