//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 282/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk282<F: Float>(t895: F, t903: F, t904: F, t912: F, t332: F, t589: F, t139: F, t215: F, t334: F, t333: F, t214: F) -> (F, F, F, F, F, F) {
    let t914 = t895 * t903 * t904;
    let t916 = F::new(0.5848223622634646207e0) * t912 * t914;
    let t917 = t589 * t332;
    let t921 = t215 * t139 * t334;
    let t923 = t333 * t921 / F::new(288.0);
    let t924 = t332 * t214;
    (t914, t916, t917, t921, t923, t924)
}
