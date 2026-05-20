//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1780/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1780<F: Float>(t111: F, t7222: F, t81437: F, t22550: F, t7031: F, t39054: F, t7025: F, t23966: F, t9231: F, t39063: F, t9239: F, t1860: F, t23992: F, t6509: F) -> (F, F, F, F, F, F, F, F) {
    let t84033 = t7222 * t111;
    let t84036 = F::new(308.0) / F::new(27.0) * t81437;
    let t84173 = t7031 * t22550;
    let t84190 = t39054 * t7025;
    let t84195 = t9231 * t23966;
    let t84216 = t39063 * t7025;
    let t84219 = t9239 * t23966;
    let t84229 = t1860 * t23992 * t6509;
    (t84033, t84036, t84173, t84190, t84195, t84216, t84219, t84229)
}
