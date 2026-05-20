//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta203<F: Float>(t25: F, t514: F, t28: F, t517: F, t1376: F, t68: F, t522: F, t9212: F, t9214: F, t3824: F, t592: F, t1287: F, t2221: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11985, t11987, t11998, t12000, t12019, t12021, t12044, t12046, t12048, t12052) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk844::<F>(t25, t514, t28, t517, t1376, t68, t522, t9212, t9214, t3824, t592, t1287, t2221);
    (t11985, t11987, t11998, t12000, t12019, t12021, t12044, t12046, t12048, t12052)
}
