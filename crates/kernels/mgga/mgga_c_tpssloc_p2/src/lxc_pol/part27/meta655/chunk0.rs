//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2286/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2286<F: Float>(t22666: F, t26189: F, t6888: F, t22892: F, t7691: F, t80645: F, t22633: F, t22635: F, t26337: F, t3911: F, t26206: F, t6883: F) -> (F, F, F, F) {
    let t90530 = t6888 * t22666 * t26189;
    let t90533 = t22892 * t80645 * t7691;
    let t90534 = F::cast_from(0.16449340668482264365e-1_f64) * t90533;
    let t90539 = t22633 * t22635 * t26337 * t3911;
    let t90541 = t6883 * t26206;
    (t90530, t90534, t90539, t90541)
}
