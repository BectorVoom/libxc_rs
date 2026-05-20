//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2035/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2035<F: Float>(t204: F, t2368: F, t2459: F, t2462: F, t2471: F, t2472: F, t2476: F, t2480: F, t2490: F, t2494: F, t2495: F, t2505: F, t2509: F, t2513: F, t268: F, t39373: F, t39389: F, t39397: F, t39400: F, t39408: F, t39411: F, t676: F, t746: F, t9489: F, t9729: F, t9734: F, t9739: F, t9755: F, t9759: F, t9766: F, t9803: F, t9810: F, t9814: F) -> F {
    let t39749 = F::cast_from(0.12842595503380418954e1_f64) * t268 * t204 * t2509 * t2513 - F::cast_from(0.21687162600603479684e-1_f64) * t268 * t2490 * t9766 - F::cast_from(0.38025319932552508021e2_f64) * t268 * t676 * t9489 * t9759 + F::cast_from(0.43374325201206959368e-1_f64) * t268 * t9803 * t2505 - F::cast_from(0.27397333333333333333e0_f64) * t268 * t204 * t2459 * t2462 - F::cast_from(0.14171548179536397724e3_f64) * t268 * t676 * t9729 * t9734 - F::cast_from(0.86748650402413918736e-1_f64) * t268 * t204 * t2368 * t2495 - F::cast_from(0.1301229756036208781e0_f64) * t268 * t9810 * t9755 + F::cast_from(0.13698666666666666666e0_f64) * t268 * t9814 * t2472 + F::cast_from(0.44060335298551228073e1_f64) * t268 * t204 * t2476 * t2480 - t39373 + t39397 + t39400 - t39408 - t39411 - F::cast_from(0.11579025239058625248e4_f64) * t9739 * t2480 * t2471 - F::cast_from(0.35089341735807877242e1_f64) * t2494 * t39389 * t746;
    t39749
}
