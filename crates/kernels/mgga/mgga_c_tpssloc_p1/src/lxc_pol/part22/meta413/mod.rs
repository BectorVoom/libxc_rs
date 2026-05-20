//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1715;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta413<F: Float>(t14850: F, t4786: F, t1117: F, t5989: F, t3313: F, t1671: F, t4781: F, t3264: F, t6024: F, t11190: F, t1098: F, t5983: F) -> (F, F, F, F, F, F, F, F) {
        let (t18676, t18677, t18679, t18680, t18682, t18683, t18685, t18686) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1715::<F>(t14850, t4786, t1117, t5989, t3313, t1671, t4781, t3264, t6024, t11190, t1098, t5983);
    (t18676, t18677, t18679, t18680, t18682, t18683, t18685, t18686)
}
