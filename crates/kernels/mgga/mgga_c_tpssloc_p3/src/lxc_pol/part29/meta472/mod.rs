//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1808;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta472<F: Float>(t1873: F, t3652: F, t652: F, t6876: F, t7000: F, t6880: F, t9348: F, t12734: F, t2314: F, t6534: F, t12739: F, t5113: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t23831, t23833, t23835, t23837, t23844, t23846, t23848, t23850, t23852) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1808::<F>(t1873, t3652, t652, t6876, t7000, t6880, t9348, t12734, t2314, t6534, t12739, t5113);
    (t23831, t23833, t23835, t23837, t23844, t23846, t23848, t23850, t23852)
}
