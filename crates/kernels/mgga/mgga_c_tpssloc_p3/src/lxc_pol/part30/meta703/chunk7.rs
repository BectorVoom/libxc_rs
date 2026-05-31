//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2292/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2292<F: Float>(t16558: F, t3: F, t17677: F, t17705: F, t1933: F, t1937: F, t23419: F, t88575: F, t88577: F, t88582: F, t88604: F, t88622: F, t88625: F, t88636: F, t88645: F) -> F {
    let t99767 = t3 * t16558;
    let t99772 = t23419 * t17705 / F::cast_from(1152.0_f64) + t88575 - t88577 + t88582 + t88604 + t23419 * t17677 / F::cast_from(1152.0_f64) + F::cast_from(0.10093189023535097714e-3_f64) * t1933 * t99767 * t1937 - t88622 + t88625 + t88636 - t88645 / F::cast_from(3456.0_f64);
    t99772
}
