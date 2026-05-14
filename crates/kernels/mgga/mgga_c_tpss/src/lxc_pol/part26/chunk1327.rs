//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1327/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1327<F: Float>(t1006: F, t4706: F, t14256: F, t20011: F, t14076: F, t64975: F, t1497: F, t8096: F, t19818: F, t18246: F, t51780: F, t3724: F, t13603: F, t1692: F, t1713: F, t17929: F, t19670: F, t19798: F, t19816: F, t20012: F, t20021: F, t20025: F, t21345: F, t2439: F, t33: F, t3552: F, t5059: F, t5586: F, t5590: F, t6149: F, t69793: F, t70213: F, t70272: F) -> (F,) {
    let t70923 = t1006 * t4706;
    let t70929 = t20011 * t14256;
    let t70932 = t64975 * t14076;
    let t70941 = t8096 * t1497;
    let t70942 = t70941 * t19818;
    let t70957 = t18246 * t51780;
    let t70960 = t1497 * t3724;
    let t70963 = 6.0 * t69793 * t20012 + 3.0 * t3552 * t1713 * t70923 + t1692 * t19798 * t1497 + 3.0 * t19670 * t70929 - 3.0 * t17929 * t70932 + t1692 * t70213 * t33 / 2.0 - t70272 + t1692 * t1713 * t13603 / 2.0 + 2.0 * t19816 * t70942 + t1692 * t5586 * t5059 / 2.0 + t1692 * t21345 * t1006 / 2.0 + 3.0 * t2439 * t6149 * t20021 + 3.0 * t2439 * t6149 * t20025 - 3.0 * t19670 * t70957 - t1692 * t5590 * t70960;
    (t70963,)
}
