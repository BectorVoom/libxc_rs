//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2339/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2339<F: Float>(t67441: F, t816: F, t20978: F, t9638: F, t20938: F, t838: F, t20953: F, t2639: F, t13177: F, t13222: F, t13262: F, t13351: F, t16839: F, t16859: F, t16912: F, t20963: F, t2643: F, t2645: F, t4167: F, t46692: F, t47285: F, t5614: F, t58569: F, t58616: F, t58668: F, t58670: F, t58853: F, t831: F, t9642: F, t9967: F) -> F {
    let t67872 = t67441 * t816;
    let t67880 = t9638 * t20978;
    let t67882 = t20938 * t838;
    let t67884 = t2639 * t20953;
    let t67898 = -t13177 * t5614 / F::cast_from(1024.0_f64) - t4167 * t16859 / F::cast_from(1024.0_f64) + t9967 * t20963 / F::cast_from(512.0_f64) - t67872 * t831 / F::cast_from(3072.0_f64) - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t58616 + F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t13262 * t13222 * t58853 * t13351 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t67880 - F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t67882 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t67884 - F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t13262 * t46692 * t47285 * t58569 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t58668 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t58670 + t9642 * t20978 / F::cast_from(256.0_f64) + t2643 * t2645 * t16839 * t16912 / F::cast_from(256.0_f64);
    t67898
}
