//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 565/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk565<F: Float>(t6635: F, t6644: F, t2047: F, t814: F, t2056: F, t2752: F) -> (F, F, F, F) {
    let t7095 = F::new(0.38381794893125283518e-1) * t6635;
    let t7097 = F::new(0.82246703342411321825e-2) * t6644;
    let t7101 = t814 * t2047;
    let t7114 = t2056 * t2752;
    (t7095, t7097, t7101, t7114)
}
