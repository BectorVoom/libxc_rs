//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2353/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2353<F: Float>(t1388: F, t25988: F, t22574: F, t26162: F, t26149: F, t6876: F, t19577: F, t31035: F, t12557: F, t1266: F, t15857: F, t1869: F, t2363: F, t26098: F, t5361: F, t6517: F, t652: F, t672: F, t6872: F, t7670: F, t90400: F, t90428: F, t90434: F, t90436: F, t90440: F, t90444: F, t90447: F, t90450: F, t90454: F, t90456: F, t91564: F) -> F {
    let t91565 = t25988 * t1388;
    let t91568 = F::new(12.0) * t22574 * t26162 * t91565;
    let t91570 = F::new(2.0) * t6876 * t26149;
    let t91573 = F::new(6.0) * t22574 * t31035 * t19577;
    let t91574 = -F::new(2.0) * t2363 * t652 * t7670 - F::new(2.0) * t12557 * t6517 - F::new(2.0) * t1266 * t26098 - t15857 * t1869 + F::new(2.0) * t5361 * t6872 - F::new(4.0) * t672 * t90400 - t90428 + t90434 - t90436 + t90440 + t90444 + t90447 - t90450 - t90454 - t90456 + t91564 + t91568 - t91570 - t91573;
    t91574
}
