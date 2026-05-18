//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 995/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk995<F: Float>(t76255: F, t76258: F, t76262: F, t14444: F, t1614: F, t5266: F, t76264: F, t76268: F, t76271: F, t76273: F, t76275: F, t76277: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77996 = F::new(0.81823984962736025192e-1) * t76255;
    let t77997 = F::new(0.40911992481368012596e-1) * t76258;
    let t77998 = F::new(0.8182398496273602519e-1) * t76262;
    let t78005 = F::new(0.11974241701863808564e0) * t5266 * t14444 * t1614;
    let t78006 = F::new(0.40911992481368012592e-1) * t76264;
    let t78007 = F::new(0.10227998120342003148e-1) * t76268;
    let t78008 = F::new(0.23948483403727617128e0) * t76271;
    let t78009 = F::new(0.72732431077987577947e-1) * t76273;
    let t78010 = F::new(0.36366215538993788973e-1) * t76275;
    let t78011 = F::new(0.13637330827122670865e-1) * t76277;
    (t77996, t77997, t77998, t78005, t78006, t78007, t78008, t78009, t78010, t78011)
}
