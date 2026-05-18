//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1161/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1161<F: Float>(t32629: F, t580: F, t1404: F, t8919: F, t131: F, t32582: F, t2240: F, t9239: F, t9231: F, t32578: F, t39063: F, t39054: F) -> (F, F, F, F, F, F, F) {
    let t117693 = t32629 * t580;
    let t117695 = t8919 * t1404;
    let t117709 = t32582 * t131;
    let t117710 = t2240 * t117709;
    let t117727 = t9239 * t117709;
    let t117734 = t9231 * t32582;
    let t117737 = t39063 * t32578;
    let t117757 = t39054 * t32578;
    (t117693, t117695, t117710, t117727, t117734, t117737, t117757)
}
