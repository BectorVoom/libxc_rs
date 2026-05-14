//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1320/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1320<F: Float>(t1288: F, t3724: F, t4806: F, t580: F, t14426: F, t30: F, t1692: F, t1989: F, t6149: F, t3610: F, t14076: F, t63840: F, t1713: F, t17929: F, t18047: F, t18052: F, t19685: F, t19802: F, t19821: F, t19829: F, t19836: F, t21270: F, t21345: F, t21353: F, t21359: F, t2439: F, t4578: F, t5539: F, t5586: F, t5590: F, t60996: F) -> (F, F) {
    let t70255 = t1288 * t3724;
    let t70258 = t580 * t4806;
    let t70261 = t30 * t14426;
    let t70272 = 2.0 * t1692 * t6149 * t1989;
    let t70286 = t1288 * t3610;
    let t70290 = t63840 * t14076;
    let t70296 = t1692 * t5586 * t4578 / 2.0 - t1692 * t5590 * t70255 + t1692 * t18052 * t70258 - t1692 * t5590 * t70261 / 2.0 + 3.0 / 2.0 * t2439 * t5586 * t21270 - t1692 * t19802 * t19821 + t70272 + t1692 * t60996 * t21353 + 3.0 * t2439 * t6149 * t19685 - t1692 * t19802 * t19836 + 3.0 / 2.0 * t2439 * t21345 * t5539 - t1692 * t18047 * t21359 / 2.0 + 3.0 * t2439 * t1713 * t70286 - 3.0 * t17929 * t70290 + 3.0 * t2439 * t6149 * t19829;
    (t70272, t70296)
}
