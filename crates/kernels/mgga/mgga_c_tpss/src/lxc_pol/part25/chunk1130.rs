//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1130/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1130<F: Float>(t1322: F, t1600: F, t1760: F, t1796: F, t1830: F, t18547: F, t19620: F, t20134: F, t20137: F, t20219: F, t20221: F, t20224: F, t20227: F, t20322: F, t3491: F, t4341: F, t544: F, t5706: F, t5799: F, t5895: F, t5910: F, t5939: F, t6243: F, t6413: F) -> (F,) {
    let t20329 = -t1322 * t5895 - t1600 * t5799 + 3.0 * t1760 * t20137 + t1760 * t20219 - t1760 * t20224 + 3.0 * t1760 * t20227 - t1796 * t4341 - t1830 * t3491 - 3.0 * t18547 * t20221 + 6.0 * t19620 * t20134 + t20322 * t544 + 3.0 * t5706 * t6413 + 3.0 * t5910 * t6243 - t5939 * t6243;
    (t20329,)
}
