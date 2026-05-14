//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1148/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1148<F: Float>(t1772: F, t18483: F, t18496: F, t19540: F, t20179: F, t20183: F, t20187: F, t20191: F, t20196: F, t20200: F, t20202: F, t20206: F, t20211: F, t20214: F, t5737: F, t5739: F, t6430: F, t6433: F) -> (F,) {
    let t20216 = -t1772 * t20214 + t18483 * t6430 - 2.0 * t18496 * t20187 - 2.0 * t19540 * t20191 + t19540 * t20202 + 2.0 * t20179 * t5739 + 2.0 * t20183 * t5739 + t20196 * t5739 + t20200 * t5739 + 2.0 * t20206 * t5739 + t20211 * t5739 - t5737 * t6433;
    (t20216,)
}
