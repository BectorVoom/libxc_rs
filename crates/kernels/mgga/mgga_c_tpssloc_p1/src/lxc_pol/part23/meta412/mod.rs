//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1229;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta412<F: Float>(t16689: F, t4101: F, t16701: F, t4205: F, t20741: F, t706: F, t20234: F, t751: F, t9897: F, t20742: F, t67: F, t758: F, t12923: F, t4194: F, t5398: F, t20800: F, t262: F, t10143: F, t20778: F, t13115: F, t16586: F, t21038: F, t225: F, t21061: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t67177, t67179, t67181, t67185, t67209) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1229::<F>(t16689, t4101, t16701, t4205, t20741, t706, t20234, t751, t9897, t20742, t67, t758);
        let (t67230, t67235, t67239, t67243, t67305, t67339) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1230::<F>(t12923, t4194, t5398, t20800, t262, t10143, t20778, t13115, t16586, t21038, t225, t21061);
    (t67177, t67179, t67181, t67185, t67209, t67230, t67235, t67239, t67243, t67305, t67339)
}
