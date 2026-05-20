//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1822/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1822<F: Float>(t39049: F, t7025: F, t39063: F, t23966: F, t9239: F, t22546: F, t22493: F, t7032: F, t23998: F, t6486: F, t1860: F, t23992: F, t6509: F) -> (F, F, F, F, F, F, F) {
    let t84209 = t39049 * t7025;
    let t84216 = t39063 * t7025;
    let t84219 = t9239 * t23966;
    let t84220 = t84219 * t22546;
    let t84222 = t22493 * t7032;
    let t84224 = t6486 * t23998;
    let t84229 = t1860 * t23992 * t6509;
    (t84209, t84216, t84219, t84220, t84222, t84224, t84229)
}
