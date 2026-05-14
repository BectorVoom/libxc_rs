//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 982/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk982<F: Float>(t1390: F, t3734: F, t22595: F, t1983: F, t6876: F, t6997: F, t191: F, t192: F, t3660: F, t2020: F, t2314: F, t6535: F, t12823: F, t1874: F, t4034: F, t6525: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22596 = t1390 * t3734;
    let t22597 = t22595 * t22596;
    let t22599 = 6.0 * t1983 * t22597;
    let t22605 = 2.0 * t6876 * t6997;
    let t22607 = t3660 * t191 * t192;
    let t22608 = t22607 * t2020;
    let t22610 = 4.0 * t2314 * t6535;
    let t22612 = 2.0 * t12823 * t1874;
    let t22614 = 4.0 * t4034 * t6525;
    (t22596, t22597, t22599, t22605, t22607, t22608, t22610, t22612, t22614)
}
