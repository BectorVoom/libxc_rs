//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1947/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1947<F: Float>(t27453: F, t27454: F, t1751: F, t477: F, t1090: F, t7362: F, t1653: F, t24858: F, t2144: F, t5011: F, t1246: F, t4733: F, t7363: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27455 = t27453 * t27454;
    let t27460 = t477 * t1751;
    let t27461 = t27460 * t1090;
    let t27462 = t7362 * t27461;
    let t27465 = t24858 * t1653;
    let t27466 = t7362 * t27465;
    let t27470 = t2144 * t5011;
    let t27471 = t27470 * t1246;
    let t27473 = t7363 * t4733;
    (t27455, t27460, t27461, t27462, t27465, t27466, t27470, t27471, t27473)
}
