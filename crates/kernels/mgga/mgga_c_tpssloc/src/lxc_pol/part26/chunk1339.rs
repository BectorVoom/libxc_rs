//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1339/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1339<F: Float>(t24525: F, t9239: F, t39063: F, t7245: F, t2108: F, t2110: F, t22531: F, t22537: F, t22546: F, t24514: F, t24520: F, t24526: F, t605: F, t607: F, t7246: F, t7256: F, t7259: F, t83745: F, t83820: F, t83822: F, t83832: F, t83835: F, t83840: F, t83846: F) -> F {
    let t85480 = t9239 * t24525;
    let t85501 = t39063 * t7245;
    let t85504 = -F::new(15.0) * t85480 * t22546 - F::new(15.0) * t24514 * t83745 + F::new(5.0) / F::new(2.0) * t24520 * t22531 + t83835 * t2110 + F::new(5.0) / F::new(2.0) * t24526 * t22531 + F::new(5.0) / F::new(2.0) * t7246 * t83840 + F::new(5.0) / F::new(6.0) * t7246 * t83846 + t605 * t607 * t2108 * t83820 + t83822 * t2110 / F::new(3.0) + t22537 * t7256 + t22537 * t7259 + F::new(35.0) * t85501 * t83832;
    t85504
}
