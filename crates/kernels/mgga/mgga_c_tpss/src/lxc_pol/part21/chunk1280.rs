//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1280/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1280<F: Float>(t19817: F, t63844: F, t14076: F, t60960: F, t17930: F, t44329: F, t1692: F, t1713: F, t17929: F, t17938: F, t18043: F, t18047: F, t18056: F, t19672: F, t19798: F, t19802: F, t19816: F, t19821: F, t2439: F, t3552: F, t35530: F, t5590: F, t580: F, t6120: F, t6121: F, t6149: F, t63814: F, t63817: F, t63823: F, t63836: F, t63837: F, t63841: F) -> (F,) {
    let t63845 = t19817 * t63844;
    let t63847 = t60960 * t14076;
    let t63850 = t17930 * t44329;
    let t63855 = t1692 * t19798 * t580 + 6.0 * t63814 * t19672 - t1692 * t5590 * t63817 / 2.0 - t1692 * t19802 * t18056 + 3.0 * t3552 * t1713 * t63823 + 3.0 * t35530 * t6121 + 3.0 / 2.0 * t2439 * t18043 * t6120 + 3.0 / 2.0 * t2439 * t6149 * t17938 + t63836 + 2.0 * t19816 * t63837 - 3.0 * t17929 * t63841 + t19816 * t63845 - 3.0 * t17929 * t63847 - 3.0 * t17929 * t63850 - t1692 * t18047 * t19821;
    (t63855,)
}
