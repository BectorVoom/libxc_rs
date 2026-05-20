//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1465/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1465<F: Float>(t1227: F, t1230: F, t15569: F, t1653: F, t19026: F, t19051: F, t22214: F, t22218: F, t22288: F, t22307: F, t248: F, t3578: F, t44828: F, t45197: F, t5005: F, t6207: F, t6211: F, t6221: F, t6227: F, t65541: F, t65703: F, t72470: F, t72495: F, t72501: F, t77961: F, t77969: F) -> F {
    let t79056 = -t19051 * t6207 / F::new(768.0) - t5005 * t22214 / F::new(1152.0) - t19051 * t6211 / F::new(384.0) - t5005 * t22218 / F::new(192.0) + t72470 / F::new(192.0) + t15569 * t22288 / F::new(36.0) - t72495 / F::new(288.0) + F::new(19.0) / F::new(288.0) * t19026 * t6221 - t72501 / F::new(288.0) - t1227 * t248 * t1230 * t77969 / F::new(768.0) - t65703 * t6227 / F::new(24.0) + F::new(55.0) / F::new(15552.0) * t1227 * t248 * t44828 * t77961 + F::new(19.0) / F::new(144.0) * t65541 * t6227 - t45197 * t3578 * t22307 * t1653 / F::new(192.0);
    t79056
}
