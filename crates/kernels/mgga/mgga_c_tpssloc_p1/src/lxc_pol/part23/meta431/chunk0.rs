//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1267/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1267<F: Float>(t15438: F, t19095: F, t19083: F, t4993: F, t18392: F, t5024: F, t1226: F, t22115: F, t1227: F, t21776: F, t248: F, t3521: F) -> (F, F, F, F, F) {
    let t72248 = t15438 * t19095;
    let t72251 = t19083 * t4993;
    let t72253 = t5024 * t18392;
    let t72255 = t22115 * t1226;
    let t72273 = t1227 * t248 * t3521 * t21776;
    (t72248, t72251, t72253, t72255, t72273)
}
