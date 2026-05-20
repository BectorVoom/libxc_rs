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
    let t67898 = -t13177 * t5614 / F::new(1024.0) - t4167 * t16859 / F::new(1024.0) + t9967 * t20963 / F::new(512.0) - t67872 * t831 / F::new(3072.0) - F::new(7.0) / F::new(384.0) * t58616 + F::new(3.0) / F::new(128.0) * t13262 * t13222 * t58853 * t13351 - F::new(7.0) / F::new(384.0) * t67880 - F::new(7.0) / F::new(4608.0) * t67882 + F::new(7.0) / F::new(4608.0) * t67884 - F::new(3.0) / F::new(512.0) * t13262 * t46692 * t47285 * t58569 - F::new(7.0) / F::new(768.0) * t58668 + F::new(7.0) / F::new(768.0) * t58670 + t9642 * t20978 / F::new(256.0) + t2643 * t2645 * t16839 * t16912 / F::new(256.0);
    t67898
}
