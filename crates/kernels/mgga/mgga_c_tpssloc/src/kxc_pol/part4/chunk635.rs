//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 635/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk635<F: Float>(t252: F, t4142: F, t1492: F, t852: F, t1493: F, t225: F, t1519: F, t798: F, t1496: F, t2563: F, t1495: F, t210: F, t776: F) -> (F, F, F, F, F, F) {
    let t4143 = t4142 * t252;
    let t4145 = t1492 * t852;
    let t4147 = t1493 * t225;
    let t4149 = t798 * t1519;
    let t4152 = t2563 * t1496;
    let t4155 = t210 * t1495 * t776;
    (t4143, t4145, t4147, t4149, t4152, t4155)
}
