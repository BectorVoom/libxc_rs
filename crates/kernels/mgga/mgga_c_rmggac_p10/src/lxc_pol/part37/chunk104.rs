//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 104/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk104<F: Float>(t247: F, t250: F, t369: F, t374: F, t179: F) -> (F, F) {
    let t416 = -F::cast_from(0.86308333333333333334e0_f64) * t247 - F::cast_from(0.301925e0_f64) * t250 - F::cast_from(0.5501625e-1_f64) * t369 - F::cast_from(0.82785e-1_f64) * t374;
    let t417 = F::cast_from(1.0_f64) / t179;
    (t416, t417)
}
