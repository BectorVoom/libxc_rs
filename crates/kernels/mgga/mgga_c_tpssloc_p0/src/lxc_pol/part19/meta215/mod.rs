//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta215 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk912;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta215<F: Float>(t10647: F, t291: F, t2784: F, t892: F, t914: F, t2787: F, t2837: F, t2841: F, t888: F, t2845: F, t10521: F, t10528: F, t10607: F, t10622: F, t10625: F, t10627: F, t10635: F) -> (F, F, F, F, F, F, F) {
        let (t10649, t10650, t10652, t10654, t10655, t10657, t10658) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk912::<F>(t10647, t291, t2784, t892, t914, t2787, t2837, t2841, t888, t2845, t10521, t10528, t10607, t10622, t10625, t10627, t10635);
    (t10649, t10650, t10652, t10654, t10655, t10657, t10658)
}
