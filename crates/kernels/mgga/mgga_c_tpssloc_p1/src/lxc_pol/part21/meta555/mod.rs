//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2250;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2251;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta555<F: Float>(t1100: F, t18730: F, t1107: F, t11243: F, t5992: F, t1102: F, t4756: F, t4764: F, t3287: F, t5999: F, t11265: F, t4748: F, t11211: F, t11372: F, t14702: F, t14705: F, t14711: F, t3270: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18731, t18742, t18746, t18747, t18749, t18751, t18752, t18754, t18755, t18757) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2250::<F>(t1100, t18730, t1107, t11243, t5992, t1102, t4756, t4764, t3287, t5999, t11265, t4748);
        let (t18759, t18761) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2251::<F>(t11211, t11372, t14702, t14705, t14711, t18742, t18747, t18749, t18752, t18755, t18757, t3270, t5999);
    (t18731, t18742, t18746, t18747, t18749, t18751, t18752, t18754, t18755, t18757, t18759, t18761)
}
