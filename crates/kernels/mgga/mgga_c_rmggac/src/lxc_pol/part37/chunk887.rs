//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 887/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk887<F: Float>(t74281: F, t74284: F, t74290: F, t74299: F, t74302: F, t74305: F, t74309: F, t74314: F, t74319: F, t76949: F, t76950: F, t76951: F, t76952: F, t76955: F, t76957: F, t76959: F, t76965: F) -> (F,) {
    let t80081 = t76949 + t76950 + t76951 - t76952 - t76955 - 0.35038612185802734374e-6 * t74281 - t74284 + t76957 - 0.87596530464506835935e-6 * t74290 + t76959 - 0.10511583655740820312e-5 * t74299 + 0.10511583655740820312e-5 * t74302 - 0.10511583655740820312e-5 * t74305 - 0.35038612185802734374e-6 * t74309 + 0.52557918278704101561e-6 * t74314 - t76965 - 0.87596530464506835932e-6 * t74319;
    (t80081,)
}
