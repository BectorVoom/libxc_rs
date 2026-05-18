//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1252/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1252<F: Float>(t25644: F, t6740: F, t1409: F, t344: F, t1009: F, t3082: F, t7586: F, t23418: F, t4669: F, t14507: F, t23536: F, t23540: F) -> (F, F, F, F, F, F) {
    let t88383 = t6740 * t25644;
    let t88449 = t1409 * t344;
    let t88451 = t6740 * t88449 * t1009;
    let t88479 = t7586 * t3082;
    let t88513 = t4669 * t23418;
    let t88594 = t14507 * t23536;
    let t88600 = t14507 * t23540;
    (t88383, t88451, t88479, t88513, t88594, t88600)
}
