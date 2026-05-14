//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1138/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1138<F: Float>(t5600: F, t921: F, t1718: F, t2668: F, t1721: F, t2746: F, t339: F, t2753: F, t5620: F, t2715: F, t2713: F, t2720: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t18076 = t5600 * t921;
    let t18079 = t1718 * t2668 / 432.0;
    let t18083 = t339 * t1721 * t2746;
    let t18086 = t5620 * t2753;
    let t18092 = t2715 * sigma0;
    let t18094 = t2713 * t18092 * t2720;
    (t18076, t18079, t18083, t18086, t18092, t18094)
}
