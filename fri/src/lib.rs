//! An implementation of the FRI low-degree test (LDT).

#![no_std]

extern crate alloc;

use crate::proof::FriProof;
use crate::prover::prove;
use crate::verifier::verify;
use core::marker::PhantomData;
use p3_challenger::Challenger;
use p3_commit::{DirectMMCS, MMCS};
use p3_field::{ExtensionField, Field};
use p3_ldt::{LDTBasedPCS, LDT};

mod proof;
mod prover;
mod verifier;

pub use proof::*;

pub struct FriLDT<F, EF, M, MC>
where
    F: Field,
    EF: ExtensionField<F>,
    M: MMCS<F>,
    MC: DirectMMCS<F>,
{
    _phantom_f: PhantomData<F>,
    _phantom_fe: PhantomData<EF>,
    _phantom_m: PhantomData<M>,
    _phantom_mc: PhantomData<MC>,
}

impl<F, EF, M, MC> LDT<F, M> for FriLDT<F, EF, M, MC>
where
    F: Field,
    EF: ExtensionField<F>,
    M: MMCS<F>,
    MC: DirectMMCS<F>,
{
    type Proof = FriProof<F, EF, M, MC>;
    type Error = ();

    fn prove<Chal>(codewords: &[M::ProverData], challenger: &mut Chal) -> Self::Proof
    where
        Chal: Challenger<F>,
    {
        prove::<F, EF, M, MC, Chal>(codewords, challenger)
    }

    fn verify<Chal>(
        _codeword_commits: &[M::Commitment],
        proof: &Self::Proof,
        challenger: &mut Chal,
    ) -> Result<(), Self::Error>
    where
        Chal: Challenger<F>,
    {
        verify::<F, EF, M, MC, Chal>(proof, challenger)
    }
}

pub type FRIBasedPCS<F, EF, M, MC> = LDTBasedPCS<F, M, FriLDT<F, EF, M, MC>>;
