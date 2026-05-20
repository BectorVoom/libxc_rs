//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1956;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta506<F: Float>(t1100: F, t21780: F, t1661: F, t5992: F, t11265: F, t21762: F, t3297: F, t136: F, t1113: F, t21769: F, t21776: F, t11219: F, t21758: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21783, t21785, t21786, t21788, t21789, t21791, t21792, t21794, t21795, t21801) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1956::<F>(t1100, t21780, t1661, t5992, t11265, t21762, t3297, t136, t1113, t21769, t21776, t11219, t21758);
    (t21783, t21785, t21786, t21788, t21789, t21791, t21792, t21794, t21795, t21801)
}
