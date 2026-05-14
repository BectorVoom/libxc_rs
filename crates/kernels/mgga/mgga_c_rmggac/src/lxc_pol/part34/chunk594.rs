//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 594/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk594<F: Float>(t14180: F, t34884: F, t7323: F, t7557: F, t14089: F, t14090: F, t49: F, t2051: F, t388: F, t14082: F, t20925: F, t253: F, t34747: F, t14083: F, t4765: F, t14030: F, t14121: F) -> (F, F, F, F, F, F, F, F) {
    let t68399 = t34884 * t14180;
    let t68401 = t7323 * t7557;
    let t68406 = t14089 * t14090 * t49;
    let t68407 = t388 * t2051;
    let t68408 = t68406 * t68407;
    let t68414 = t253 * t34747 * t14082 * t20925 * t2051;
    let t68417 = t4765 * t14083 * t49;
    let t68418 = t68417 * t68407;
    let t68420 = t14030 * t14121;
    (t68399, t68401, t68406, t68408, t68414, t68417, t68418, t68420)
}
