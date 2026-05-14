//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1283/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1283<F: Float>(t1006: F, t3683: F, t823: F, t1497: F, t2116: F, t20047: F, t63884: F, t18246: F, t63859: F, t44350: F, t1692: F, t1713: F, t17929: F, t18047: F, t18268: F, t18271: F, t19670: F, t19802: F, t19816: F, t20012: F, t20054: F, t33: F, t3552: F, t5678: F, t61264: F, t6214: F, t63787: F, t63790: F, t63814: F, t63836: F, t64237: F, t64277: F) -> (F,) {
    let t64914 = t823 * t1006 * t3683;
    let t64917 = t1497 * t2116;
    let t64923 = t20047 * t63884;
    let t64928 = t18246 * t63859;
    let t64941 = t20047 * t44350;
    let t64944 = -t63787 - t1692 * t61264 * t6214 / 2.0 + t63790 + 6.0 * t19670 * t64914 + 3.0 * t3552 * t1713 * t64917 - t1692 * t19802 * t18268 + 6.0 * t17929 * t64923 + 6.0 * t63814 * t20012 - 6.0 * t19670 * t64928 - t1692 * t18047 * t20054 - t1692 * t19802 * t18271 / 2.0 - t63836 - t1692 * t64277 * t5678 + t1692 * t64237 * t33 / 2.0 + 2.0 * t19816 * t64941;
    (t64944,)
}
