//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta511<F: Float>(t13030: F, t225: F, t13062: F, t13378: F, t193: F, t2379: F, t16465: F, t12250: F, t1824: F, t1799: F, t3791: F, t3850: F) -> (F, F, F, F, F, F, F, F) {
        let (t47585, t47609, t47618, t47645, t53866, t54014, t54068, t54153) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1759::<F>(t13030, t225, t13062, t13378, t193, t2379, t16465, t12250, t1824, t1799, t3791, t3850);
    (t47585, t47609, t47618, t47645, t53866, t54014, t54068, t54153)
}
