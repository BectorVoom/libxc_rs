//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1515/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1515<F: Float>(t12419: F, t19871: F, t19956: F, t20448: F, t20463: F, t20468: F, t3803: F, t3805: F, t39936: F, t5246: F, t74120: F, t74258: F, t74260: F, t74274: F, t74276: F, t74297: F, t74299: F, t74360: F, t74376: F, t74393: F) -> F {
    let t80352 = F::new(7.0) / F::new(96.0) * t74258 + F::new(7.0) / F::new(96.0) * t74260 - t5246 * t3805 * t74120 * t20468 / F::new(32.0) - F::new(7.0) / F::new(48.0) * t74274 + F::new(35.0) / F::new(96.0) * t74276 + t39936 + F::new(7.0) / F::new(1152.0) * t74297 + F::new(7.0) / F::new(1152.0) * t74299 + F::new(7.0) / F::new(3.0) * t74360 + F::new(7.0) / F::new(384.0) * t74376 - F::new(5.0) / F::new(128.0) * t3803 * t12419 * t19956 * t20448 + t3803 * t3805 * t19871 * t20463 / F::new(128.0) - F::new(7.0) / F::new(4.0) * t74393;
    t80352
}
