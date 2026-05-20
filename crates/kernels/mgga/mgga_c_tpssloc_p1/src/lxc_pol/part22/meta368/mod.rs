//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta368 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1616;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1617;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta368<F: Float>(t17375: F, t17449: F, t17516: F, t17558: F, t300: F, t2940: F, t5808: F, t10629: F, t5774: F, t10632: F, t950: F, t959: F, t225: F, t5849: F, t1603: F, t4657: F, t1634: F, t4693: F, t3174: F, t5851: F, t17183: F, t977: F, t17178: F, t2979: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17561, t17563, t17564, t17565, t17566, t17568) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1616::<F>(t17375, t17449, t17516, t17558, t300, t2940, t5808, t10629, t5774, t10632, t950, t959);
        let (t17575, t17579, t17583, t17588, t17593, t17596) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1617::<F>(t225, t5849, t1603, t4657, t1634, t4693, t3174, t5851, t17183, t977, t17178, t2979);
    (t17561, t17563, t17564, t17565, t17566, t17568, t17575, t17579, t17583, t17588, t17593, t17596)
}
