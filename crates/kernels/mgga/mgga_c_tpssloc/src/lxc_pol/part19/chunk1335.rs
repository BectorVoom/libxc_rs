//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1335/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1335<F: Float>(t11647: F, t1203: F, t11859: F, t1222: F, t11797: F, t3490: F, t11172: F, t1227: F, t248: F, t3521: F, t11801: F, t204: F, t486: F, t1213: F, t1216: F, t11862: F, t13969: F) -> (F, F, F, F, F, F, F) {
    let t45002 = t1203 * t11647;
    let t45007 = t11859 * t1222;
    let t45009 = t3490 * t11797;
    let t45013 = t1227 * t248 * t3521 * t11172;
    let t45015 = t3490 * t11801;
    let t45017 = t204 * t486;
    let t45020 = t1213 * t248 * t45017 * t1216;
    let t45027 = t1227 * t13969 * t11862;
    (t45002, t45007, t45009, t45013, t45015, t45020, t45027)
}
