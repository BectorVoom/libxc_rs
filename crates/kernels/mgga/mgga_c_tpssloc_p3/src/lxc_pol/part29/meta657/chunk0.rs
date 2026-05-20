//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2183/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2183<F: Float>(t26135: F, t5113: F, t1983: F, t23857: F, t7753: F, t24991: F, t6876: F, t25992: F, t22592: F, t7685: F, t22948: F, t5161: F) -> (F, F, F, F, F, F) {
    let t90410 = F::new(4.0) * t5113 * t26135;
    let t90418 = F::new(2.0) * t1983 * t7753 * t23857;
    let t90421 = F::new(6.0) * t6876 * t24991;
    let t90428 = F::new(2.0) * t6876 * t25992;
    let t90434 = F::new(6.0) * t7685 * t22592;
    let t90436 = t1983 * t22948 * t5161;
    (t90410, t90418, t90421, t90428, t90434, t90436)
}
