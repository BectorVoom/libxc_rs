//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1936;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta618<F: Float>(t16215: F, t221: F, t91194: F, t6604: F, t80893: F, t1361: F, t6925: F, t6976: F, t22828: F, t26243: F, t26271: F, t80779: F, t22844: F, t7708: F, t16391: F, t26309: F, t5259: F, t80820: F, t16265: F, t22833: F, t5293: F, t80816: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t91196, t91200, t91204, t91206) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1936::<F>(t16215, t221, t91194, t6604, t80893, t1361, t6925, t6976, t22828, t26243, t26271, t80779);
        let (t91210, t91212, t91214, t91216, t91218) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1937::<F>(t22844, t6976, t22828, t7708, t16391, t26309, t5259, t80820, t16265, t22833, t5293, t80816);
    (t91196, t91200, t91204, t91206, t91210, t91212, t91214, t91216, t91218)
}
