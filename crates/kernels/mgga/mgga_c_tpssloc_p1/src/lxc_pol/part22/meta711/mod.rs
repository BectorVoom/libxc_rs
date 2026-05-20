//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta711 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2308;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2309;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta711<F: Float>(t58021: F, t46278: F, t1484: F, t4303: F, t16634: F, t4205: F, t40738: F, t40754: F, t12895: F, t2522: F, t40741: F, t40743: F, t40748: F, t40760: F, t4307: F, t5544: F, t40761: F, t16689: F, t4101: F, t16701: F, t20741: F, t706: F, t708: F, t20234: F, t751: F, t9897: F, t13133: F, t5597: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t67162, t67163, t67169, t67170, t67174, t67175) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2308::<F>(t58021, t46278, t1484, t4303, t16634, t4205, t40738, t40754, t12895, t2522, t40741, t40743, t40748, t40760, t4307, t5544);
        let (t67176, t67178, t67180, t67183, t67186, t67191) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2309::<F>(t40761, t16689, t4101, t16701, t4205, t20741, t706, t708, t20234, t751, t9897, t13133, t5597);
    (t67162, t67163, t67169, t67170, t67174, t67175, t67176, t67178, t67180, t67183, t67186, t67191)
}
