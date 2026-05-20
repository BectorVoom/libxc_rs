//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1598;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta422<F: Float>(t22740: F, t3792: F, t22897: F, t1992: F, t22751: F, t6892: F, t6883: F, t6908: F, t3719: F, t6890: F, t6889: F, t6888: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t22898, t22899, t22900, t22907, t22908, t22909, t22910, t22916, t22917, t22918) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1598::<F>(t22740, t3792, t22897, t1992, t22751, t6892, t6883, t6908, t3719, t6890, t6889, t6888);
    (t22898, t22899, t22900, t22907, t22908, t22909, t22910, t22916, t22917, t22918)
}
