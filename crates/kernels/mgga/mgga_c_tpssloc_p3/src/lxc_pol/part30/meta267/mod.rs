//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta267<F: Float>(t1307: F, t6968: F, t6637: F, t6888: F, t2009: F, t794: F, t6897: F, t1338: F, t6604: F) -> (F, F, F, F, F, F) {
        let (t6969, t6970, t6971, t6973, t6975, t6976) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1214::<F>(t1307, t6968, t6637, t6888, t2009, t794, t6897, t1338, t6604);
    (t6969, t6970, t6971, t6973, t6975, t6976)
}
