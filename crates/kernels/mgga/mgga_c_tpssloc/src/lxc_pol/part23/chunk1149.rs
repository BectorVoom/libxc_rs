//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1149/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1149<F: Float>(t135: F, t21545: F, t973: F, t13995: F, t18041: F, t17659: F, t4644: F, t10422: F, t21573: F, t3070: F, t1036: F, t21483: F, t1041: F, t13969: F, t21511: F, t10413: F, t21531: F) -> (F, F, F, F, F, F, F) {
    let t70665 = t973 * t135 * t21545;
    let t70703 = t13995 * t18041;
    let t70711 = t4644 * t17659;
    let t70724 = t3070 * t10422 * t21573;
    let t70766 = t21483 * t1036;
    let t70792 = t1041 * t13969 * t21511;
    let t70800 = t10413 * t10422 * t21531;
    (t70665, t70703, t70711, t70724, t70766, t70792, t70800)
}
