//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 906/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk906<F: Float>(t27444: F, t3961: F, t24601: F, t24574: F, t8067: F, t1184: F, t1715: F, t24745: F, t7363: F, t1751: F, t477: F, t1090: F, t7362: F, t1653: F, t24858: F, t2144: F, t5011: F) -> (F, F, F, F, F, F, F, F) {
    let t27445 = t27444 * t3961;
    let t27446 = t24601 * t27445;
    let t27451 = t24574 * t8067;
    let t27453 = t1715 * t1184;
    let t27454 = t24745 * t7363;
    let t27455 = t27453 * t27454;
    let t27460 = t477 * t1751;
    let t27461 = t27460 * t1090;
    let t27462 = t7362 * t27461;
    let t27465 = t24858 * t1653;
    let t27466 = t7362 * t27465;
    let t27470 = t2144 * t5011;
    (t27445, t27446, t27451, t27453, t27455, t27462, t27466, t27470)
}
