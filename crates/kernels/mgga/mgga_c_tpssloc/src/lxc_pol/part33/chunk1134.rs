//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1134/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1134<F: Float>(t1921: F, t7577: F, t14202: F, t6765: F, t14159: F, t6717: F, t23509: F, t25682: F, t25644: F, t25650: F, t6740: F, t1409: F, t344: F, t1009: F, t3082: F, t7586: F) -> (F, F, F, F, F, F, F, F) {
    let t88162 = t7577 * t1921;
    let t88321 = t6765 * t14202;
    let t88336 = t6717 * t14159;
    let t88342 = t23509 * t25682;
    let t88372 = t25650 * t25644;
    let t88383 = t6740 * t25644;
    let t88449 = t1409 * t344;
    let t88451 = t6740 * t88449 * t1009;
    let t88479 = t7586 * t3082;
    (t88162, t88321, t88336, t88342, t88372, t88383, t88451, t88479)
}
