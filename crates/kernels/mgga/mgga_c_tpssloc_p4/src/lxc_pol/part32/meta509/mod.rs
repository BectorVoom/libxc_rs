//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta509<F: Float>(t26248: F, t559: F, t1358: F, t7715: F, t1831: F, t22783: F, t5234: F, t6951: F, t1369: F, t22788: F, t5314: F, t6952: F) -> (F, F, F, F, F, F, F) {
        let (t26249, t26251, t26255, t26257, t26258, t26260, t26262) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1835::<F>(t26248, t559, t1358, t7715, t1831, t22783, t5234, t6951, t1369, t22788, t5314, t6952);
    (t26249, t26251, t26255, t26257, t26258, t26260, t26262)
}
