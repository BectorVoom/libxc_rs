//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1005/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1005<F: Float>(t10377: F, t10385: F, t10480: F, t10876: F, t10883: F, t14508: F, t14511: F, t17612: F, t17616: F, t21393: F, t21398: F, t21405: F, t21483: F, t21487: F, t21490: F, t21493: F, t3130: F, t378: F, t5875: F, t5880: F, t973: F) -> F {
    let t21498 = t14508 * t5875 / F::new(512.0) + t10480 * t21393 / F::new(512.0) - t10876 * t21398 / F::new(512.0) - t14511 * t5880 / F::new(1024.0) + t10883 * t21405 / F::new(3072.0) + t10377 + t21483 * t378 / F::new(3072.0) + t10385 + t3130 * t21487 / F::new(512.0) - t973 * t21490 / F::new(48.0) + t973 * t21493 / F::new(72.0) + t17612 / F::new(1536.0) + t17616 / F::new(288.0);
    t21498
}
