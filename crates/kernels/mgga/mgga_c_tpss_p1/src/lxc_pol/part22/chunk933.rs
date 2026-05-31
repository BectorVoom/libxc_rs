//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 933/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk933<F: Float>(t355: F, t9036: F, t215: F, t334: F, t68: F, t333: F, t219: F, t2769: F, t979: F, t73: F, t8549: F, t8552: F) -> (F, F, F, F, F) {
    let t9038 = t355 * t9036 / F::cast_from(10368.0_f64);
    let t9040 = t215 * t68 * t334;
    let t9042 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t333 * t9040;
    let t9058 = t2769 * t219;
    let t9065 = t979 * t979;
    let t9066 = F::cast_from(1.0_f64) / t9065;
    let t9067 = t73 * t9066;
    let t9076 = t8549 * t8552;
    (t9038, t9042, t9058, t9067, t9076)
}
