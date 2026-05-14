//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1337/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1337<F: Float>(t1011: F, t1212: F, t44706: F, t11692: F, t11693: F, t11697: F, t11853: F, t1213: F, t248: F, t3570: F, t11163: F, t1227: F, t3521: F, t221: F, t44483: F, t456: F) -> (F, F, F, F, F) {
    let t45080 = t44706 * t1011 * t1212;
    let t45086 = t11692 * t11697 * t11693;
    let t45102 = t1213 * t248 * t3570 * t11853;
    let t45108 = t1227 * t248 * t3521 * t11163;
    let t45112 = 5.0 / 486.0 * t456 * t221 * t44483;
    (t45080, t45086, t45102, t45108, t45112)
}
