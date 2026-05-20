//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1399/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1399<F: Float>(t121774: F, t870: F, t1914: F, t4303: F, t2752: F, t33465: F, t193: F, t200: F, t8565: F, t115009: F, t118393: F, t1408: F, t1877: F, t24191: F, t25: F, t25015: F, t25021: F, t25373: F, t26744: F, t26756: F, t30767: F, t31430: F, t31448: F, t33466: F, t33476: F, t606: F, t6671: F, t7114: F, t81547: F, t98064: F) -> (F, F, F, F, F) {
    let t121775 = t121774 * t870;
    let t121779 = t1914 * t4303;
    let t121782 = t33465 * t2752;
    let t121789 = t193 * t200 * t8565;
    let t121798 = t1877 * t31430 * t1408 / F::new(2.0) - t1877 * t7114 * t118393 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t24191 * t81547 * t33476 + t26756 * t98064 * t31448 + t1877 * t121775 * t25 / F::new(2.0) + t26756 * t25373 * t121779 - t1877 * t121782 * t6671 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t115009 * t25021 + F::new(3.0) * t121789 * t25015 + t1877 * t33466 * t606 / F::new(2.0) - t1877 * t26744 * t30767 / F::new(2.0);
    (t121775, t121779, t121782, t121789, t121798)
}
