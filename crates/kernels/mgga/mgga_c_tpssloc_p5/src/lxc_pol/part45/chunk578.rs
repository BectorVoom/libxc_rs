//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 578/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk578<F: Float>(t1065: F, t6705: F, t6704: F, t1945: F, t990: F, t131: F, t6679: F, t1926: F, t995: F, t1919: F, t210: F, t1929: F, rho0: F) -> (F, F, F, F, F, F, F) {
    let t6706 = t6705 * t1065;
    let t6707 = t6704 * t6706;
    let t6710 = t990 * t1945;
    let t6712 = t6679 * t131;
    let t6716 = t1926 * t995 / F::cast_from(288.0_f64);
    let t6717 = t1919 * t210;
    let t6720 = t1929 * rho0;
    (t6706, t6707, t6710, t6712, t6716, t6717, t6720)
}
