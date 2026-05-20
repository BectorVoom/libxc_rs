//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2772/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2772<F: Float>(t5572: F, t9541: F, t4233: F, t776: F, t5527: F, t828: F, t5611: F, t5624: F, t9601: F, t1512: F, t47092: F, t119: F, t13222: F, t13228: F, t210: F, t2571: F, t2643: F, t2647: F, t41009: F, t41053: F, t4178: F, t46587: F, t46595: F, t46611: F, t46616: F, t46618: F, t46644: F, t46649: F, t46658: F, t47039: F, t58090: F) -> (F, F) {
    let t58550 = t9541 * t5572;
    let t58552 = t776 * t4233;
    let t58557 = t5527 * t828;
    let t58569 = t5611 * t828;
    let t58574 = t9601 * t5624;
    let t58576 = t47092 * t1512;
    let t58581 = F::new(7.0) / F::new(576.0) * t46587 + F::new(35.0) / F::new(72.0) * t41009 - F::new(7.0) / F::new(576.0) * t46595 + t2571 * t210 * t119 * t58090 / F::new(8.0) - F::new(35.0) / F::new(216.0) * t58550 - t4178 * t13222 * t13228 * t58552 / F::new(96.0) + F::new(5.0) / F::new(64.0) * t2643 * t47039 * t58557 * t2647 + F::new(7.0) / F::new(144.0) * t46611 - F::new(7.0) / F::new(288.0) * t46616 - F::new(7.0) / F::new(576.0) * t46618 - t4178 * t13222 * t13228 * t46644 / F::new(192.0) + t2643 * t13222 * t58569 * t2647 / F::new(384.0) + F::new(595.0) / F::new(3456.0) * t58574 - F::new(119.0) / F::new(6912.0) * t58576 + F::new(119.0) / F::new(864.0) * t46649 - F::new(119.0) / F::new(1728.0) * t41053 - F::new(7.0) / F::new(288.0) * t46658;
    (t58552, t58581)
}
