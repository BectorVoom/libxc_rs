//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1481/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1481<F: Float>(t11814: F, t3572: F, t11825: F, t3523: F, t11820: F, t3536: F, t11778: F, t121: F, t11148: F, t1227: F, t248: F, t11728: F, t11729: F, t3570: F) -> (F, F, F, F, F) {
    let t45262 = t11814 * t3572;
    let t45264 = t11825 * t3523;
    let t45266 = t3536 * t11820;
    let t45268 = t121 * t11778;
    let t45271 = t1227 * t248 * t45268 * t11148;
    let t45283 = t11728 * t248 * t3570 * t11729;
    (t45262, t45264, t45266, t45271, t45283)
}
