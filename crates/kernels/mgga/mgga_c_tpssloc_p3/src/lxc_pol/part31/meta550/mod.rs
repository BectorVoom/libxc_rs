//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1777;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1778;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta550<F: Float>(t23171: F, t23228: F, t6572: F, t212: F, t6554: F, t852: F, t23030: F, t23253: F, t6555: F, t81573: F, t6563: F, t81597: F, t794: F, t23208: F, t1882: F, t81686: F, t9537: F, t213: F, t225: F, t6556: F, t81632: F, t23012: F, t6573: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t82082, t82087, t82099, t82120, t82122) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1777::<F>(t23171, t23228, t6572, t212, t6554, t852, t23030, t23253, t6555, t81573, t6563, t81597);
        let (t82133, t82147, t82153, t82159, t82209, t82211) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1778::<F>(t794, t852, t23030, t23208, t1882, t81686, t9537, t213, t225, t6556, t81632, t23012, t6573);
    (t82082, t82087, t82099, t82120, t82122, t82133, t82147, t82153, t82159, t82209, t82211)
}
