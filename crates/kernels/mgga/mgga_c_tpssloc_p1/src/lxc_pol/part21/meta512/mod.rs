//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2159;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta512<F: Float>(t17161: F, t2979: F, t10214: F, t17152: F, t1040: F, t5904: F, t248: F, t3101: F, t5867: F, t1020: F, t10372: F, t10377: F, t10381: F, t10385: F, t1046: F, t13750: F, t13758: F, t13767: F, t13946: F, t17593: F, t17596: F, t973: F) -> (F, F, F, F, F) {
        let (t17599, t17602, t17607, t17611, t17614) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2159::<F>(t17161, t2979, t10214, t17152, t1040, t5904, t248, t3101, t5867, t1020, t10372, t10377, t10381, t10385, t1046, t13750, t13758, t13767, t13946, t17593, t17596, t973);
    (t17599, t17602, t17607, t17611, t17614)
}
