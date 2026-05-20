//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1697;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta501<F: Float>(t1799: F, t26395: F, t6637: F, t6888: F, t1998: F, t6434: F, t214: F, t1985: F, t19739: F, t550: F, t6976: F, t1992: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28148, t28149, t28150, t28159, t28160, t28161, t28163, t28164, t28165) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1697::<F>(t1799, t26395, t6637, t6888, t1998, t6434, t214, t1985, t19739, t550, t6976, t1992);
    (t28148, t28149, t28150, t28159, t28160, t28161, t28163, t28164, t28165)
}
