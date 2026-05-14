//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1248/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1248<F: Float>(t19818: F, t20047: F, t1006: F, t1398: F, t33: F, t3724: F, t1497: F, t750: F, t821: F, t1692: F, t1713: F, t17929: F, t18047: F, t19670: F, t19798: F, t19802: F, t19816: F, t19841: F, t20012: F, t20018: F, t20021: F, t20025: F, t20041: F, t2439: F, t5586: F, t5590: F, t5671: F, t5678: F, t6149: F, t6207: F, t6214: F) -> (F, F, F, F, F, F) {
    let t20048 = t20047 * t19818;
    let t20050 = t1006 * t1398;
    let t20054 = t33 * t3724;
    let t20058 = t1497 * t750;
    let t20065 = t1497 * t821;
    let t20069 = 3.0 * t19670 * t20012 + 3.0 / 2.0 * t2439 * t5586 * t6207 - 3.0 / 2.0 * t17929 * t20018 + 3.0 / 2.0 * t2439 * t1713 * t20021 + 3.0 / 2.0 * t2439 * t1713 * t20025 + 3.0 / 2.0 * t2439 * t6149 * t5671 + t1692 * t19798 * t33 / 2.0 - t1692 * t19802 * t5678 / 2.0 + t1692 * t6149 * t1006 / 2.0 - 3.0 / 2.0 * t17929 * t20041 - t1692 * t18047 * t6214 / 2.0 + t19816 * t20048 - t1692 * t5590 * t20050 / 2.0 - t1692 * t5590 * t20054 / 2.0 + 3.0 / 2.0 * t2439 * t1713 * t20058 + t1692 * t5586 * t1497 / 2.0 - t1692 * t5590 * t20065 / 2.0 - t19841;
    (t20048, t20050, t20054, t20058, t20065, t20069)
}
