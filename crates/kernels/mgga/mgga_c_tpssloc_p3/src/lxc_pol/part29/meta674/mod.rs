//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta674 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2262;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta674<F: Float>(t24987: F, t6880: F, t22573: F, t7684: F, t22575: F, t22585: F, t7685: F, t22607: F, t7754: F, t6875: F, t8944: F, t26164: F, t1983: F, t22578: F, t7753: F, t7756: F, t531: F, t7752: F, t22596: F, t16153: F, t24995: F, t8945: F, t22574: F, t25988: F, t31035: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t91642, t91657, t91662, t91666, t91671) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2262::<F>(t24987, t6880, t22573, t7684, t22575, t22585, t7685, t22607, t7754, t6875, t8944, t26164);
        let (t91673, t91674, t91678, t91681, t91684) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2263::<F>(t1983, t22578, t7753, t22607, t7756, t531, t7752, t22596, t16153, t24995, t8945, t22574, t25988, t31035);
    (t91642, t91657, t91662, t91666, t91671, t91673, t91674, t91678, t91681, t91684)
}
