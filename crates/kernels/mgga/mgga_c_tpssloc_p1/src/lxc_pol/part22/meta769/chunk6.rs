//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2616/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2616<F: Float>(t15594: F, t15737: F, t18342: F, t18594: F, t19058: F, t19101: F, t5005: F, t5024: F, t53372: F, t53399: F, t6207: F, t6227: F, t6232: F, t66406: F, t66408: F, t66410: F, t66413: F, t66437: F) -> F {
    let t73019 = t5024 * t19101 / F::new(288.0) + t53372 * t6227 / F::new(512.0) - t53399 * t6232 / F::new(1024.0) - t15594 * t6207 / F::new(1536.0) - t5005 * t19101 / F::new(1536.0) + F::new(5.0) / F::new(6912.0) * t66406 - t66408 / F::new(144.0) + F::new(19.0) / F::new(864.0) * t66410 - t5005 * t18594 / F::new(256.0) - F::new(5.0) / F::new(432.0) * t5024 * t18342 + t15737 * t19058 / F::new(512.0) + t66413 / F::new(384.0) + t66437 / F::new(256.0);
    t73019
}
