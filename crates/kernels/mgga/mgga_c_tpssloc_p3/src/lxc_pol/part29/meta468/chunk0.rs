//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1798/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1798<F: Float>(t23237: F, t6555: F, t6552: F, t2379: F, t6554: F, t6553: F, t23035: F, t6547: F, t6568: F, t23030: F, t6563: F, t6567: F, t794: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23238 = t23237 * t6555;
    let t23239 = t6552 * t23238;
    let t23241 = t6554 * t2379;
    let t23242 = t6553 * t23241;
    let t23243 = t23035 * t23242;
    let t23249 = t6547 * t6568;
    let t23250 = F::cast_from(0.38381794893125283518e-1_f64) * t23249;
    let t23251 = t23030 * t6563;
    let t23252 = F::cast_from(0.26044789391763585244e-1_f64) * t23251;
    let t23253 = t794 * t6567;
    (t23238, t23239, t23241, t23242, t23243, t23249, t23250, t23252, t23253)
}
