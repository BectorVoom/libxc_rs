//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2064;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2065;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta599<F: Float>(t23168: F, t23223: F, t1882: F, t81686: F, t9537: F, t213: F, t225: F, t852: F, t23164: F, t23204: F, t23222: F, t23238: F, t23196: F, t6562: F, t23202: F, t6556: F, t81632: F, t23012: F, t6573: F, t1883: F, t82045: F, t6555: F, t82133: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t82150, t82154, t82159, t82172, t82174) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2064::<F>(t23168, t23223, t1882, t81686, t9537, t213, t225, t852, t23164, t23204, t23222, t23238);
        let (t82182, t82197, t82209, t82211, t82219, t82221) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2065::<F>(t23196, t23204, t6562, t225, t23202, t6556, t81632, t23012, t6573, t1883, t82045, t23164, t6555, t82133);
    (t82150, t82154, t82159, t82172, t82174, t82182, t82197, t82209, t82211, t82219, t82221)
}
