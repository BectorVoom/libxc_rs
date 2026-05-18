//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 485/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk485<F: Float>(t1454: F, t626: F, t1453: F, t2331: F, t1444: F, t2341: F, t1449: F, t2349: F, t1409: F, t2433: F, t2440: F, t1472: F, t751: F) -> (F, F, F, F, F, F, F) {
    let t4041 = t626 * t1454;
    let t4043 = t2331 * t1453;
    let t4049 = t2341 * t1444;
    let t4059 = t2349 * t1449;
    let t4080 = t2433 * t1409;
    let t4087 = t2440 * t1409;
    let t4100 = t1472 * t751;
    (t4041, t4043, t4049, t4059, t4080, t4087, t4100)
}
