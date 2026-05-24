//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 96/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk96<F: Float>(t121: F, t348: F, t305: F, t321: F, t326: F, t333: F, t344: F) -> (F, F) {
    let t349 = t121 * t348;
    let t352 = F::cast_from(0.19957069503106347607e-1_f64) * t305 * t321 - F::cast_from(0.19957069503106347607e-1_f64) * t326 * t333 + F::cast_from(0.26552308210121162678e-3_f64) * t344 * t321 - F::cast_from(0.26552308210121162678e-3_f64) * t349 * t333;
    (t349, t352)
}
