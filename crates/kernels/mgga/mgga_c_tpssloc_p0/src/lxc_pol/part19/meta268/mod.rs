//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta268 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1023;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta268<F: Float>(t11712: F, t11880: F, t11720: F, t491: F, t11721: F, t6739: F, t10471: F, t3502: F, t3508: F, t11624: F, t3612: F, t1215: F, t3590: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11881, t11882, t11883, t11884, t11887, t11888, t11889, t11890, t11893, t11896) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1023::<F>(t11712, t11880, t11720, t491, t11721, t6739, t10471, t3502, t3508, t11624, t3612, t1215, t3590);
    (t11881, t11882, t11883, t11884, t11887, t11888, t11889, t11890, t11893, t11896)
}
