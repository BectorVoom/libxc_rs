//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1876;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1877;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta537<F: Float>(t27536: F, t7377: F, t24833: F, t8073: F, t5068: F, t8082: F, t5079: F, t221: F, t4899: F, t2127: F, t2135: F, t477: F, t3242: F, t491: F) -> (F, F, F, F, F, F, F, F) {
        let (t27537, t27540, t27543, t27546, t27548, t27549) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1876::<F>(t27536, t7377, t24833, t8073, t5068, t8082, t5079, t221, t4899, t2127);
        let t27550 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1877::<F>(t2135, t477);
        let t27551 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1878::<F>(t3242, t491);
    (t27537, t27540, t27543, t27546, t27548, t27549, t27550, t27551)
}
