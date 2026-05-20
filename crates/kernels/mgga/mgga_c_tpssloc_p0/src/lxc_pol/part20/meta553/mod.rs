//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2099;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2100;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta553<F: Float>(t2635: F, t41424: F, t2639: F, t9663: F, t13258: F, t9634: F, t9629: F, t6589: F, t67: F, t246: F, t232: F, t9458: F, t10046: F, t814: F, t225: F, t9520: F, t10647: F, t892: F, t2784: F, t2841: F, t22715: F, t268: F, t271: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41425, t41427, t41435, t41437, t41466, t41467, t41468) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2099::<F>(t2635, t41424, t2639, t9663, t13258, t9634, t9629, t6589, t67, t246, t232, t9458);
        let (t41520, t41554, t41618, t41623, t41654) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2100::<F>(t10046, t814, t225, t9520, t10647, t892, t2784, t2841, t22715, t268, t271);
    (t41425, t41427, t41435, t41437, t41466, t41467, t41468, t41520, t41554, t41618, t41623, t41654)
}
