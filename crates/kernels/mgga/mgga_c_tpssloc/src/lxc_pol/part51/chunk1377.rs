//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1377/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1377<F: Float>(t118414: F, t118455: F, t118467: F, t118954: F, t121258: F, t121264: F, t121271: F, t121275: F, t121279: F, t22960: F, t24191: F, t25373: F, t25375: F, t26756: F, t31442: F, t33483: F, t86716: F, t86770: F, t92319: F) -> F {
    let t121283 = -F::new(3.0) / F::new(2.0) * t92319 * t31442 - F::new(3.0) * t26756 * t86716 * t121258 - t121264 - F::new(3.0) / F::new(2.0) * t24191 * t118467 + t26756 * t86770 * t33483 - F::new(3.0) / F::new(2.0) * t24191 * t118455 + t121271 * t25375 + t26756 * t118414 + t26756 * t118954 + F::new(3.0) * t24191 * t25373 * t121275 - F::new(3.0) / F::new(2.0) * t24191 * t22960 * t121279;
    t121283
}
