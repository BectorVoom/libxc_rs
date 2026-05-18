//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1033/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1033<F: Float>(t30663: F, t6555: F, t6552: F, t6572: F, t1880: F, t30626: F, t30630: F, t30637: F, t30640: F, t30645: F, t30647: F, t30651: F, t30655: F, t30659: F, t30662: F, t6627: F, t6632: F, t855: F) -> (F, F, F) {
    let t30664 = t30663 * t6555;
    let t30666 = F::new(0.3289868133696452873e-1) * t6552 * t30664;
    let t30667 = t30663 * t6572;
    let t30669 = F::new(0.16449340668482264365e-1) * t1880 * t30667;
    let t30670 = F::new(4.0) * t30630 * t855 + F::new(2.0) * t30647 * t855 - F::new(6.0) * t30651 * t855 + F::new(4.0) * t6627 * t6632 + t30626 + t30637 - t30640 + t30645 - t30655 - t30659 + t30662 - t30666 - t30669;
    (t30664, t30667, t30670)
}
