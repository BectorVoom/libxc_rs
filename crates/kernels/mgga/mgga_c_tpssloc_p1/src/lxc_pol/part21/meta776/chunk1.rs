//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2686/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2686<F: Float>(t1388: F, t5356: F, t15899: F, t39570: F, t39585: F, t39590: F, t39593: F, t5160: F, t56391: F, t56393: F, t56395: F, t56396: F, t56398: F, t56401: F, t56403: F) -> F {
    let t56404 = t1388 * t5356;
    let t56408 = F::new(8.0) * t15899 * t5160 * t56404 + t39570 - t39585 + t39590 - t39593 + t56391 - t56393 + t56395 + t56396 + t56398 + t56401 + t56403;
    t56408
}
