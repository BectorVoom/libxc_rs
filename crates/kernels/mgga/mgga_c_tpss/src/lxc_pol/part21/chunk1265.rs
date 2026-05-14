//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1265/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1265<F: Float>(t18107: F, t2677: F, t5610: F, t8471: F, t5620: F, t8480: F, t1718: F, t9040: F, t8430: F, t18083: F, t2753: F, t8953: F, t2713: F, t5608: F, t8970: F, t18092: F, t8550: F, t8557: F) -> (F, F, F, F, F, F, F, F, F) {
    let t61318 = t18107 * t2677;
    let t61322 = t5610 * t8471;
    let t61324 = t5620 * t8480;
    let t61329 = 5.0 / 1296.0 * t1718 * t9040;
    let t61334 = t5610 * t8430;
    let t61336 = t18083 * t2753;
    let t61341 = t5620 * t8953;
    let t61344 = t2713 * t5608 * t8970;
    let t61350 = t8550 * t18092 * t8557;
    (t61318, t61322, t61324, t61329, t61334, t61336, t61341, t61344, t61350)
}
