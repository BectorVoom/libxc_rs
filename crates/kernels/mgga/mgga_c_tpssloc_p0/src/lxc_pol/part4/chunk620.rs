//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 620/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk620<F: Float>(t1388: F, t1390: F, t531: F, t571: F, t112: F, t1395: F, t111: F, t576: F, t2218: F, t2221: F, t2225: F, t2232: F) -> (F, F, F, F, F) {
    let t3919 = t1388 * t1390;
    let t3924 = t531 * t571;
    let t3938 = t1395 * t112;
    let t3941 = t576 * t111;
    let t3951 = -t2218 - F::cast_from(0.78e0_f64) * t2221 - F::cast_from(0.578e2_f64) * t2225 + t2232;
    (t3919, t3924, t3938, t3941, t3951)
}
