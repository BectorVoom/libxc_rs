//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta240 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk895;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta240<F: Float>(t1041: F, t17659: F, t4630: F, t4641: F, t248: F, t3101: F, t5873: F, t3130: F, t376: F, t5866: F, t2970: F, t5824: F, t973: F, t5828: F, t10231: F, t5817: F, t2989: F, t5398: F, t2987: F, t5836: F, t5842: F, t13847: F, t4514: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17660, t17662, t17667, t17668, t17712, t17763) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk895::<F>(t1041, t17659, t4630, t4641, t248, t3101, t5873, t3130, t376, t5866, t2970, t5824);
        let (t17764, t17770, t17784, t17794, t17800, t17804, t17808) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk896::<F>(t17763, t973, t2970, t5828, t10231, t5817, t2989, t5398, t2987, t5836, t5842, t13847, t4514);
    (t17660, t17662, t17667, t17668, t17712, t17764, t17770, t17784, t17794, t17800, t17804, t17808)
}
