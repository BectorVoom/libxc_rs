//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2039;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta544<F: Float>(t2223: F, t3826: F, t11985: F, t25: F, t514: F, t11998: F, t28: F, t517: F, t32253: F, t59: F, t154: F, t541: F, t12289: F, t1336: F, t835: F, t12364: F, t3777: F, t1314: F, t9569: F, t1329: F, t2559: F, t3732: F, t12214: F, t782: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39857, t39861, t39877, t39933, t39934, t39936) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2039::<F>(t2223, t3826, t11985, t25, t514, t11998, t28, t517, t32253, t59, t154, t541);
        let (t39944, t39947, t40005, t40006, t40018, t40021) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2040::<F>(t12289, t1336, t835, t12364, t3777, t1314, t9569, t1329, t2559, t3732, t12214, t782);
    (t39857, t39861, t39877, t39933, t39934, t39936, t39944, t39947, t40005, t40006, t40018, t40021)
}
