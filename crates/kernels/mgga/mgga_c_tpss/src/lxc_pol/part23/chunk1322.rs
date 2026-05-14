//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1322/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1322<F: Float>(t1861: F, t19411: F, t19414: F, t19417: F, t5966: F, t5976: F, t5979: F, t65234: F, t65237: F, t65244: F, t65293: F, t65296: F, t65299: F, t65302: F, t19345: F, t5975: F) -> (F, F) {
    let t67990 = 5.0 / 6.0 * t5966 * t65293 + t65296 * t1861 / 3.0 + 2.0 / 3.0 * t65299 * t1861 + 2.0 / 3.0 * t65302 * t1861 + 2.0 / 3.0 * t19411 * t5976 + 2.0 / 3.0 * t19411 * t5979 + t65234 * t1861 / 3.0 + 2.0 / 3.0 * t65237 * t1861 + 2.0 / 3.0 * t19414 * t5976 + 2.0 / 3.0 * t19414 * t5979 + t65244 * t1861 / 3.0 + 2.0 / 3.0 * t19417 * t5976;
    let t68003 = t5975 * t19345;
    (t67990, t68003)
}
