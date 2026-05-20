//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2649/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2649<F: Float>(t12282: F, t5234: F, t3809: F, t120: F, t16205: F, t12283: F, t16227: F, t1351: F, t5286: F, t12429: F, t1352: F, t16148: F, t16224: F, t16305: F, t16308: F, t16311: F, t16314: F, t16401: F, t3803: F, t3805: F, t3807: F, t39945: F, t39948: F, t39950: F, t39956: F, t39958: F, t39960: F, t40197: F, t5246: F) -> (F, F) {
    let t53945 = t5234 * t12282;
    let t53946 = t53945 * t3809;
    let t53958 = t120 * t16205;
    let t53965 = t12283 * t16227;
    let t53973 = t5286 * t1351;
    let t53978 = -F::new(7.0) / F::new(192.0) * t53946 + F::new(7.0) / F::new(768.0) * t39945 - F::new(119.0) / F::new(2304.0) * t39948 - F::new(119.0) / F::new(4608.0) * t39950 + F::new(7.0) / F::new(1536.0) * t39956 - F::new(7.0) / F::new(768.0) * t39958 + F::new(7.0) / F::new(1536.0) * t39960 - t5246 * t16305 * t16311 * t40197 / F::new(128.0) + t3803 * t3805 * t53958 * t3807 / F::new(256.0) - t16401 * t16314 / F::new(64.0) + F::new(35.0) / F::new(192.0) * t53965 - F::new(5.0) / F::new(128.0) * t3803 * t16224 * t16148 * t1352 + t12429 * t16308 / F::new(128.0) + t3803 * t16305 * t53973 * t3807 / F::new(128.0);
    (t53958, t53978)
}
