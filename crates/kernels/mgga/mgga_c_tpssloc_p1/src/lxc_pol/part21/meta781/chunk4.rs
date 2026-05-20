//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2715/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2715<F: Float>(t39563: F, t39570: F, t39585: F, t39590: F, t39593: F, t39595: F, t56388: F, t56391: F, t56393: F, t56395: F, t56396: F, t56398: F, t56401: F, t56403: F, t56411: F, t56412: F, t56416: F, t56417: F) -> F {
    let t57201 = t39563 - t56388 + t39570 + t56391 - t56393 + t56395 + t56396 + t56398 + t56401 + t56403 - t39585 + t39590 - t39593 + t39595 + t56411 - t56412 + t56416 - t56417;
    t57201
}
