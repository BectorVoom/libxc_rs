//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1099/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1099<F: Float>(t18434: F, t18461: F, t219: F, t5919: F, t1219: F, t1838: F) -> (F, F, F, F) {
    let t18934 = 35.0 / 216.0 * t18434;
    let t18943 = 119.0 / 3456.0 * t18461;
    let t18950 = t5919 * t219;
    let t18967 = t1219 * t1838;
    (t18934, t18943, t18950, t18967)
}
