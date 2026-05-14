//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 391/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk391<F: Float>(t4528: F, t973: F, t1597: F, t2987: F, t1604: F, t225: F, t1539: F, t248: F, t3051: F, t1041: F, t135: F, t1606: F, t1036: F, t1612: F, t1616: F, t3101: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4529 = t973 * t4528;
    let t4531 = t2987 * t1597;
    let t4557 = t1604 * t225;
    let t4571 = t248 * t3051 * t1539;
    let t4572 = t1041 * t4571;
    let t4603 = t135 * t1606;
    let t4604 = t973 * t4603;
    let t4625 = t1612 * t1036;
    let t4630 = t248 * t3101 * t1616;
    (t4529, t4531, t4557, t4571, t4572, t4603, t4604, t4625, t4630)
}
