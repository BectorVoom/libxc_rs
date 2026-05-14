//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1236/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1236<F: Float>(t121506: F, t6552: F, t6637: F, t776: F, t118677: F, t118679: F, t118682: F, t118694: F, t118695: F, t118699: F, t118700: F, t118710: F, t118715: F, t118719: F, t121488: F, t121493: F, t121498: F, t121501: F, t121504: F, t812: F, t829: F) -> (F,) {
    let t121509 = t6552 * t6637 * t121506 * t776;
    let t121511 = t118677 + t118679 + t118682 + t118694 + t118695 + t118699 - t812 * t121488 * t829 + t118700 + 0.16449340668482264365e-1 * t121493 + 0.16449340668482264365e-1 * t121498 + 0.82246703342411321825e-2 * t121501 - 0.41123351671205660912e-2 * t121504 - 0.16449340668482264365e-1 * t121509 - t118710 - t118715 + t118719;
    (t121511,)
}
