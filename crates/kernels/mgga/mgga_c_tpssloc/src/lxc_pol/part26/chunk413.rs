//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 413/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk413<F: Float>(t1914: F, t202: F, t193: F, t870: F, t1915: F, t28: F, t1877: F, t1268: F, t1873: F, t191: F, t513: F, t192: F) -> (F, F, F, F, F, F, F) {
    let t1962 = t202 * t1914;
    let t1964 = t193 * t1962 * t870;
    let t1969 = t1915 * t28;
    let t1971 = t1877 * t1969 / F::new(2.0);
    let t1979 = F::new(2.0) * t1268 * t1873;
    let t1982 = t513 * t191;
    let t1983 = t1982 * t192;
    (t1962, t1964, t1969, t1971, t1979, t1982, t1983)
}
