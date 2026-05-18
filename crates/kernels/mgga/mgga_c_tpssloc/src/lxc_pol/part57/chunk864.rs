//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 864/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk864<F: Float>(t1459: F, t1849: F, t2040: F, t31532: F, t33085: F, t33199: F, t33204: F, t33208: F, t33213: F, t33216: F, t33218: F, t33224: F, t33227: F, t6517: F, t652: F, t7042: F, t7472: F, t7802: F, t8604: F) -> F {
    let t33228 = -F::new(2.0) * t1459 * t31532 + t1849 * t8604 - F::new(2.0) * t2040 * t33085 - F::new(2.0) * t33204 * t652 - F::new(2.0) * t6517 * t7802 - F::new(2.0) * t7042 * t7472 - t33199 - t33208 - t33213 - t33216 - t33218 + t33224 - t33227;
    t33228
}
