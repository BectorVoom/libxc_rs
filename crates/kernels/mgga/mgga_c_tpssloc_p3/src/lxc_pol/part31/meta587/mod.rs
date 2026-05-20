//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1829;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta587<F: Float>(t26322: F, t80855: F, t91152: F, t236: F, t26318: F, t91005: F, t22782: F, t5234: F, t1369: F, t7712: F, t80939: F, t22683: F, t26285: F, t6546: F, t26289: F, t6604: F, t80887: F, t16060: F, t6951: F, t1878: F, t80730: F, t80893: F, t6925: F, t6976: F, t26271: F, t80779: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t91154, t91158, t91160, t91161, t91167, t91170) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1829::<F>(t26322, t80855, t91152, t236, t26318, t91005, t22782, t5234, t1369, t7712, t80939, t22683, t26285, t6546);
        let (t91179, t91191, t91194, t91198, t91202, t91206) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1830::<F>(t26289, t6604, t80887, t16060, t6951, t1878, t80730, t80893, t6925, t6976, t26271, t80779);
    (t91154, t91158, t91160, t91161, t91167, t91170, t91179, t91191, t91194, t91198, t91202, t91206)
}
