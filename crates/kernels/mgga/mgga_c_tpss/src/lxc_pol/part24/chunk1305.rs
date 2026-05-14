//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1305/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1305<F: Float>(t17930: F, t69863: F, t1364: F, t555: F, t63783: F, t17929: F, t4578: F, t821: F, t1398: F, t3724: F, t19817: F, t1288: F, t1692: F, t1713: F, t19670: F, t19678: F, t19798: F, t19816: F, t19819: F, t21255: F, t21266: F, t21345: F, t2439: F, t3552: F, t5586: F, t5590: F, t580: F, t64284: F, t69838: F, t69842: F, t69848: F, t69851: F, t69857: F, t69858: F) -> (F, F, F) {
    let t69864 = t17930 * t69863;
    let t69868 = t63783 * t555 * t1364;
    let t69870 = 6.0 * t17929 * t69868;
    let t69871 = t4578 * t821;
    let t69881 = t1398 * t3724;
    let t69882 = t19817 * t69881;
    let t69885 = 3.0 * t2439 * t5586 * t21266 + 3.0 * t3552 * t1713 * t69838 + 3.0 * t19670 * t69842 - 3.0 * t64284 * t19678 + 3.0 * t17929 * t69848 + 2.0 * t69851 * t19819 - t69857 - 3.0 / 2.0 * t17929 * t69858 + t1692 * t19798 * t1288 - 3.0 / 2.0 * t17929 * t69864 + t69870 - t1692 * t5590 * t69871 / 2.0 + 3.0 * t3552 * t5586 * t21255 + t1692 * t21345 * t580 / 2.0 + 2.0 * t19816 * t69882;
    (t69870, t69881, t69885)
}
