//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1163;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1164;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1165;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta248<F: Float>(t25: F, t868: F, t1877: F, t1915: F, t2522: F, t606: F, t6542: F, t6666: F, t6670: F, t221: F, t60: F, t3: F, t607: F, t343: F, t984: F, t3034: F, t334: F, t371: F, t202: F, t6665: F, t193: F, t776: F, t870: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t6671, t6678, t6686, t6729) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1163::<F>(t25, t868, t1877, t1915, t2522, t606, t6542, t6666, t6670, t221, t60, t3, t607);
        let (t6733, t6739) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1164::<F>(t343, t984, t3034, t334);
        let (t6793, t6794, t6829, t6834) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1165::<F>(t334, t371, t202, t6665, t1877, t1915, t193, t2522, t6670, t776, t868, t870);
    (t6671, t6678, t6686, t6729, t6733, t6739, t6793, t6794, t6829, t6834)
}
