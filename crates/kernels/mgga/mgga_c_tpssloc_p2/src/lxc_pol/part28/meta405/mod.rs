//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1567;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1568;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta405<F: Float>(t1307: F, t1377: F, t1385: F, t22635: F, t22633: F, t154: F, t835: F, t3748: F, t212: F, t562: F) -> (F, F, F, F, F, F) {
        let (t22637, t22638, t22639, t22641) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1567::<F>(t1307, t1377, t1385, t22635, t22633, t154, t835);
        let t22642 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1568::<F>(t22641, t3748);
        let t22643 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1569::<F>(t212, t562);
    (t22637, t22638, t22639, t22641, t22642, t22643)
}
