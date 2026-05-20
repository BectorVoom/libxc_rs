//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2253/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2253<F: Float>(t23097: F, t232: F, t67793: F, t815: F, t2628: F, t5585: F, t776: F, t13228: F, t4233: F, t6605: F, t25119: F, t58557: F) -> (F, F, F, F) {
    let t98758 = t23097 * t815 * t67793 * t232;
    let t98762 = t23097 * t2628 * t5585 * t776;
    let t98766 = t6605 * t2628 * t13228 * t4233;
    let t98770 = t25119 * t815 * t58557 * t232;
    (t98758, t98762, t98766, t98770)
}
