//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1173/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1173<F: Float>(t2814: F, t5648: F, t1735: F, t9133: F, t10514: F, t1692: F, t1713: F, t18042: F, t18047: F, t18052: F, t198: F, t207: F, t2116: F, t2133: F, t2428: F, t2433: F, t2439: F, t3552: F, t5586: F, t5590: F, t750: F, t821: F, t823: F) -> (F, F, F) {
    let t18196 = t5648 * t2814;
    let t18200 = t1735 * t9133;
    let t18230 = t18042 * t198 * t207 * t823 - 6.0 * t10514 * t2439 * t5590 - 2.0 * t1692 * t18047 * t821 + 2.0 * t1692 * t18052 * t2433 - t1692 * t2428 * t5590 + 6.0 * t1713 * t2116 * t3552 + 3.0 * t1713 * t2133 * t2439 + 6.0 * t2439 * t5586 * t750;
    (t18196, t18200, t18230)
}
