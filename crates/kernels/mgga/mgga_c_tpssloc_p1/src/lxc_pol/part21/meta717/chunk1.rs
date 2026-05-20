//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2559/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2559<F: Float>(t10489: F, t4644: F, t10898: F, t4630: F, t10882: F, t48569: F, t13961: F, t3109: F, t13542: F, t2970: F, t973: F, t13546: F) -> (F, F, F, F, F, F) {
    let t50183 = t4644 * t10489;
    let t50189 = t10898 * t4630;
    let t50193 = t48569 * t10882;
    let t50229 = t3109 * t13961;
    let t50242 = t973 * t2970 * t13542;
    let t50250 = t973 * t2970 * t13546;
    (t50183, t50189, t50193, t50229, t50242, t50250)
}
