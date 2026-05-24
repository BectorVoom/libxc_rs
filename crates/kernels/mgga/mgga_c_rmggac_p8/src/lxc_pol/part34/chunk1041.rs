//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1041/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1041<F: Float>(t76264: F, t76268: F, t76271: F, t76273: F, t76275: F, t76277: F, t76279: F, t321: F, t5148: F, t77992: F, t77995: F, t77996: F, t77997: F, t77998: F, t77999: F, t78005: F) -> F {
    let t78006 = F::cast_from(0.40911992481368012592e-1_f64) * t76264;
    let t78007 = F::cast_from(0.10227998120342003148e-1_f64) * t76268;
    let t78008 = F::cast_from(0.23948483403727617128e0_f64) * t76271;
    let t78009 = F::cast_from(0.72732431077987577947e-1_f64) * t76273;
    let t78010 = F::cast_from(0.36366215538993788973e-1_f64) * t76275;
    let t78011 = F::cast_from(0.13637330827122670865e-1_f64) * t76277;
    let t78012 = F::cast_from(0.13637330827122670865e-1_f64) * t76279;
    let t78013 = -t77992 - t77995 - t77996 - t77997 - t77998 - F::cast_from(0.11974241701863808564e0_f64) * t5148 * t77999 * t321 + t78005 - t78006 - t78007 - t78008 + t78009 + t78010 - t78011 - t78012;
    t78013
}
