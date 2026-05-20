//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1163;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta274<F: Float>(t107: F, t2585: F, t2281: F, t667: F, t2333: F, t626: F, t2359: F, t655: F, t93: F, t94: F, t101: F, t102: F, t195: F, t40: F, t197: F, t52: F, t138: F, t2409: F, t125: F, t2412: F, t701: F, t2414: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9358, t9359, t9361, t9363, t9365, t9384, t9397) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1163::<F>(t107, t2585, t2281, t667, t2333, t626, t2359, t655, t93, t94, t101, t102);
        let (t9398, t9427, t9438, t9454, t9457) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1164::<F>(t9397, t195, t40, t197, t52, t138, t2409, t125, t2412, t701, t2414);
    (t9358, t9359, t9361, t9363, t9365, t9384, t9398, t9427, t9438, t9454, t9457)
}
