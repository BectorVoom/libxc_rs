//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 269/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk269<F: Float>(t837: F, t861: F, t141: F, t839: F, t850: F, t852: F, t855: F, t860: F) -> (F, F, F) {
    let t862 = t861 * t837;
    let t863 = t141 * t862;
    let t865 = F::new(0.1898925e1) * t850 - t852 - F::new(0.29896666666666666667e0) * t839 + F::new(0.3071625e0) * t855 - t860 - F::new(0.82156666666666666667e-1) * t863;
    (t862, t863, t865)
}
