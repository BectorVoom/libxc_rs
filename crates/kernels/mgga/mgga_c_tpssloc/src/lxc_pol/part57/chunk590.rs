//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 590/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk590<F: Float>(t8343: F, t8344: F, t1894: F, t1902: F, t214: F, t1880: F, t1268: F, t8326: F, t191: F, t1980: F, t192: F) -> (F, F, F, F, F, F, F) {
    let t8345 = t8343 * t8344;
    let t8356 = t1894 * t1902;
    let t8357 = t214 * t8356;
    let t8359 = 0.16449340668482264365e-1 * t1880 * t8357;
    let t8445 = t1268 * t8326;
    let t8446 = 2.0 * t8445;
    let t8449 = t1980 * t191;
    let t8450 = t8449 * t192;
    (t8345, t8356, t8357, t8359, t8446, t8449, t8450)
}
