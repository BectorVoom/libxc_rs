//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1350/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1350<F: Float>(t1622: F, t1935: F, t21444: F, t21532: F, t21538: F, t21566: F, t21597: F, t23419: F, t25577: F, t28526: F, t343: F, t5869: F, t6717: F, t6734: F, t6755: F, t7578: F, t83215: F, t99590: F, t99624: F, t99631: F, t99731: F) -> F {
    let t106267 = t99590 / F::new(768.0) - t99624 / F::new(144.0) + t25577 * t5869 / F::new(512.0) + F::new(0.30279567070605293142e-3) * t99631 + t6755 * t21597 / F::new(1536.0) + t99731 * t1622 / F::new(768.0) - F::new(0.10093189023535097714e-3) * t1935 * t21444 * t343 * t6734 - F::new(0.30279567070605293142e-3) * t28526 * t7578 - t6717 * t21538 / F::new(36.0) - t83215 * t21532 / F::new(768.0) + t23419 * t21566 / F::new(768.0);
    t106267
}
