//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1255/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1255<F: Float>(t135: F, t21537: F, t973: F, t21541: F, t21545: F, t13995: F, t18041: F, t17659: F, t4644: F, t10422: F, t21573: F, t3070: F) -> (F, F, F, F, F, F) {
    let t70655 = t973 * t135 * t21537;
    let t70660 = t973 * t135 * t21541;
    let t70665 = t973 * t135 * t21545;
    let t70703 = t13995 * t18041;
    let t70711 = t4644 * t17659;
    let t70724 = t3070 * t10422 * t21573;
    (t70655, t70660, t70665, t70703, t70711, t70724)
}
