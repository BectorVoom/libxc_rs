//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2190/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2190<F: Float>(t11801: F, t3490: F, t204: F, t486: F, t1213: F, t1216: F, t248: F, t11862: F, t1227: F, t13969: F, t11716: F, t44833: F, t44834: F) -> (F, F, F, F, F) {
    let t45015 = t3490 * t11801;
    let t45017 = t204 * t486;
    let t45020 = t1213 * t248 * t45017 * t1216;
    let t45027 = t1227 * t13969 * t11862;
    let t45030 = t44833 * t11716 * t44834;
    (t45015, t45017, t45020, t45027, t45030)
}
