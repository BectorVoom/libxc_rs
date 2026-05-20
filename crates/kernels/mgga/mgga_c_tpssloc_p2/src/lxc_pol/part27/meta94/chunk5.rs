//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 615/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk615<F: Float>(t1941: F, t354: F, t1927: F, t1935: F, t1937: F, t378: F) -> (F, F) {
    let t1942 = t354 * t1941;
    let t1945 = t1927 / F::new(96.0) + F::cast_from(0.10093189023535097714e-3_f64) * t1935 * t1937 + t1942 * t378 / F::new(1536.0);
    (t1942, t1945)
}
