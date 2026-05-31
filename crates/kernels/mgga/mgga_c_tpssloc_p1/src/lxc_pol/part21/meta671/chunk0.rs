//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2474/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2474<F: Float>(t1227: F, t248: F, t3243: F, t45046: F, t221: F, t44483: F, t456: F, t3575: F, t42386: F, t11888: F, t11914: F, t11784: F, t820: F) -> (F, F, F, F, F, F) {
    let t45049 = t1227 * t248 * t45046 * t3243;
    let t45112 = F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t456 * t221 * t44483;
    let t45113 = t3575 * t42386;
    let t45114 = t11888 * t45113;
    let t45119 = t11914 * t45113;
    let t45124 = t820 * t11784;
    (t45049, t45112, t45113, t45114, t45119, t45124)
}
