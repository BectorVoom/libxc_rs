//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta364<F: Float>(t2644: F, t820: F, t1509: F, t828: F, t2647: F, t2632: F, t776: F, t1500: F, t2693: F, t4163: F, t838: F, t120: F, t4233: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13223, t13224, t13225, t13228, t13229, t13230, t13231, t13234, t13237, t13242) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1467::<F>(t2644, t820, t1509, t828, t2647, t2632, t776, t1500, t2693, t4163, t838, t120, t4233);
    (t13223, t13224, t13225, t13228, t13229, t13230, t13231, t13234, t13237, t13242)
}
