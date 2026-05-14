//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 375/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk375<F: Float>(t1751: F, t466: F, t1734: F, t491: F, t1246: F, t493: F, t1244: F, t1729: F, t470: F, t494: F) -> (F, F, F, F, F) {
    let t1752 = t466 * t1751;
    let t1755 = t491 * t1734;
    let t1756 = t1755 * t1246;
    let t1758 = t493 * t1751;
    let t1760 = t1244 * t1756 + t1729 * t494 + t1758 * t470;
    (t1752, t1755, t1756, t1758, t1760)
}
