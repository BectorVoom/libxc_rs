//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2075;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2076;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta650<F: Float>(t91137: F, t26297: F, t80853: F, t80855: F, t26301: F, t1831: F, t80866: F, t131: F, t6931: F, t9537: F, t26322: F, t236: F, t26318: F, t91005: F, t22782: F, t5234: F, t1369: F, t7712: F, t80939: F, t22683: F, t26285: F, t6546: F, t26289: F, t6604: F, t80887: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t91138, t91141, t91144, t91149, t91155, t91158) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2075::<F>(t91137, t26297, t80853, t80855, t26301, t1831, t80866, t131, t6931, t9537, t26322, t236, t26318, t91005);
        let (t91159, t91160, t91162, t91167, t91171, t91179) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2076::<F>(t91158, t22782, t5234, t1369, t7712, t80939, t22683, t26285, t6546, t26289, t6604, t80887);
    (t91138, t91141, t91144, t91149, t91155, t91159, t91160, t91162, t91167, t91171, t91179)
}
