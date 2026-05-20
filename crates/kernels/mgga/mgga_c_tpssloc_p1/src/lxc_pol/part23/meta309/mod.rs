//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1058;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1059;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta309<F: Float>(t1100: F, t21780: F, t1661: F, t5992: F, t11265: F, t21762: F, t3297: F, t136: F, t1113: F, t21769: F, t21776: F, t11219: F, t21758: F, t11243: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t21781: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21783, t21785, t21786, t21788, t21789, t21791, t21792, t21794, t21795, t21801) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1058::<F>(t1100, t21780, t1661, t5992, t11265, t21762, t3297, t136, t1113, t21769, t21776, t11219, t21758);
        let (t21802, t21804, t21808) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1059::<F>(t136, t21801, t11243, t21785, t21760, t21764, t21767, t21771, t21774, t21778, t21781, t21783, t21786, t21789, t21792, t21795);
    (t21783, t21786, t21788, t21789, t21791, t21792, t21794, t21795, t21801, t21802, t21804, t21808)
}
