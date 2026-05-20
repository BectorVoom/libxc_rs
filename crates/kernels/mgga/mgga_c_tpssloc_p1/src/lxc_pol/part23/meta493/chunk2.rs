//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1514/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1514<F: Float>(t1363: F, t1367: F, t19904: F, t20433: F, t3870: F, t40070: F, t5240: F, t53901: F, t6427: F, t6431: F, t74191: F, t74212: F, t74214: F, t74217: F, t74228: F, t74256: F, t79921: F, t79984: F, t80021: F, t820: F) -> F {
    let t80330 = -F::new(35.0) / F::new(96.0) * t74191 + F::new(595.0) / F::new(648.0) * t53901 + F::new(7.0) / F::new(384.0) * t74212 + F::new(7.0) / F::new(192.0) * t74214 + F::new(7.0) / F::new(384.0) * t74217 - F::new(7.0) / F::new(192.0) * t74228 + F::new(35.0) / F::new(128.0) * t1363 * t40070 * t820 * t80021 + F::new(5.0) / F::new(256.0) * t1363 * t3870 * t820 * t79921 + F::new(5.0) / F::new(128.0) * t19904 * t6427 - t1363 * t1367 * t820 * t79984 / F::new(768.0) - F::new(5.0) / F::new(32.0) * t5240 * t20433 - t19904 * t6431 / F::new(128.0) + F::new(35.0) / F::new(48.0) * t74256;
    t80330
}
