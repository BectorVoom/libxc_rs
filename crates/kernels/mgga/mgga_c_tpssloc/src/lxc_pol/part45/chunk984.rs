//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 984/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk984<F: Float>(t23222: F, t31366: F, t6552: F, t225: F, t31362: F, t23030: F, t31405: F, t23270: F, t2379: F, t25038: F, t31337: F, t31315: F, t6562: F, t794: F) -> (F, F, F, F, F) {
    let t114808 = t6552 * t31366 * t23222;
    let t114811 = t31362 * t225;
    let t114814 = t23030 * t31405;
    let t114815 = F::new(0.26044789391763585244e-1) * t114814;
    let t114822 = t25038 * t23270 * t31337 * t2379;
    let t114827 = t6562 * t794 * t31315;
    (t114808, t114811, t114815, t114822, t114827)
}
