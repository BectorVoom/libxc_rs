//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta164 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk768;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta164<F: Float>(t334: F, t371: F, t533: F, t556: F, t1433: F, t71: F, t1458: F, t89: F, t1597: F, t343: F, t88: F, t2130: F, rho1: F, t590: F, t60: F, t93: F, t101: F, t584: F, t16: F, t2: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6793, t6924, t7445, t7458, t7577, t7676, t8025) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk768::<F>(t334, t371, t533, t556, t1433, t71, t1458, t89, t1597, t343, t88, t2130, rho1);
        let (t8705, t9108, t9174, t9211, t9212) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk769::<F>(t590, t60, t93, t101, t584, t16, t2);
    (t6793, t6924, t7445, t7458, t7577, t7676, t8025, t8705, t9108, t9174, t9211, t9212)
}
