//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1865;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta617<F: Float>(t1985: F, t22666: F, t28205: F, t7700: F, t90739: F, t28206: F, t6883: F, t1385: F, t1992: F, t22635: F, t3886: F, t6460: F, t22674: F, t6897: F, t22892: F, t28209: F, t22685: F, t28191: F, t6888: F, t19631: F, t6889: F, t6890: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t96857, t96866, t96868, t96873) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1865::<F>(t1985, t22666, t28205, t7700, t90739, t28206, t6883, t1385, t1992, t22635, t3886, t6460);
        let (t96878, t96893, t96896, t96900, t96905) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1866::<F>(t22674, t28205, t6897, t22892, t28209, t22666, t22685, t28191, t6888, t19631, t6889, t6890);
    (t96857, t96866, t96868, t96873, t96878, t96893, t96896, t96900, t96905)
}
