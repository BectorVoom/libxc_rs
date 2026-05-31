//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2653/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2653<F: Float>(t12429: F, t1352: F, t16394: F, t1825: F, t19815: F, t19871: F, t19882: F, t19956: F, t19972: F, t19986: F, t20442: F, t3803: F, t3805: F, t3807: F, t5245: F, t5248: F, t5252: F, t5287: F, t56817: F, t74090: F, t74110: F, t74120: F) -> F {
    let t74133 = t3803 * t3805 * t74090 * t3807 / F::cast_from(768.0_f64) - t12429 * t20442 / F::cast_from(1024.0_f64) - t3803 * t5248 * t56817 * t1825 / F::cast_from(1024.0_f64) - t3803 * t5248 * t19956 * t5287 / F::cast_from(1024.0_f64) + t16394 * t19986 / F::cast_from(256.0_f64) + t19815 * t5245 * t5252 / F::cast_from(512.0_f64) - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t74110 - t3803 * t5248 * t74090 * t1352 / F::cast_from(3072.0_f64) + t16394 * t19882 / F::cast_from(256.0_f64) - t16394 * t19972 / F::cast_from(512.0_f64) + t3803 * t3805 * t74120 * t3807 / F::cast_from(768.0_f64) - t3803 * t5248 * t19871 * t5287 / F::cast_from(1024.0_f64) - t3803 * t5248 * t74120 * t1352 / F::cast_from(3072.0_f64);
    t74133
}
