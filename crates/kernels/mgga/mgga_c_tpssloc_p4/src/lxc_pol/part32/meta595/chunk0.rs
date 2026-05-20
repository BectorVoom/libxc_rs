//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1983/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1983<F: Float>(t1484: F, t4233: F, t5544: F, t828: F, t1215: F, t5398: F, t1388: F, t6347: F, t1799: F, t5356: F, t1351: F, t5286: F) -> (F, F, F, F, F, F, F) {
    let t67783 = t1484 * t4233;
    let t67793 = t5544 * t828;
    let t72164 = t5398 * t1215;
    let t74032 = t6347 * t1388;
    let t74060 = t1799 * t5356;
    let t74366 = t6347 * t1351;
    let t74677 = t1799 * t5286;
    (t67783, t67793, t72164, t74032, t74060, t74366, t74677)
}
