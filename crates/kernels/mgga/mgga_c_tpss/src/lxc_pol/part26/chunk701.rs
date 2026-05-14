//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 701/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk701<F: Float>(t30: F, t33: F, t187: F, t4377: F, t1288: F, t3217: F, t1197: F, t2: F, t555: F, t580: F, t1497: F, t3225: F, t1201: F, t1006: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t4379 = 0.19751673498613801407e-1 * t4377 * t187;
    let t4380 = t3217 * t1288;
    let t4383 = t1197 * t2;
    let t4387 = piecewise3(t31, 0.0, -2.0 / 9.0 * t4380 * t580 + 4.0 / 3.0 * t4383 * t555);
    let t4388 = t3225 * t1497;
    let t4391 = t1201 * t2;
    let t4395 = piecewise3(t34, 0.0, -2.0 / 9.0 * t4388 * t1006 - 4.0 / 3.0 * t4391 * t555);
    let t4397 = t4387 / 2.0 + t4395 / 2.0;
    (t4379, t4380, t4388, t4397)
}
