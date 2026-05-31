//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 930/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk930<F: Float>(t3590: F, t72: F, t732: F, t2222: F, t3560: F, t1289: F, t724: F, t581: F, t3564: F, t3589: F, t680: F, t2345: F, t3557: F) -> (F, F, F, F, F) {
    let t10684 = t3590 * t72;
    let t10686 = F::cast_from(0.36622894612013090108e-3_f64) * t10684 * t732;
    let t10687 = t3560 * t2222;
    let t10689 = t724 * t1289;
    let t10690 = t10689 * t581;
    let t10692 = F::cast_from(24.0_f64) * t3564 * t10690;
    let t10698 = t680 * t3589;
    let t10701 = t3557 * t2345;
    (t10686, t10687, t10692, t10698, t10701)
}
