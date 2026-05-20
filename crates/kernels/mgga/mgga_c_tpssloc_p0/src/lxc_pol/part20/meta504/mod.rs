//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2013;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta504<F: Float>(t9108: F, t94: F, t102: F, t9174: F, t12512: F, t580: F, t1404: F, t3931: F, t1395: F, t3946: F, t12537: F, t576: F, t2: F, t591: F, t21: F, t9: F, t587: F, t598: F, t14: F, t2230: F, t594: F, t9223: F, t22811: F, t19: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t35577, t35761, t39022, t39024, t39026, t39028) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2013::<F>(t9108, t94, t102, t9174, t12512, t580, t1404, t3931, t1395, t3946, t12537, t576);
        let (t39031, t39033, t39035, t39037, t39039, t39043) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2014::<F>(t2, t591, t21, t9, t587, t598, t14, t2230, t594, t9223, t22811, t19);
    (t35577, t35761, t39022, t39024, t39026, t39028, t39031, t39033, t39035, t39037, t39039, t39043)
}
