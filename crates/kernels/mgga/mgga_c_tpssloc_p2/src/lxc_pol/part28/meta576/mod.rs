//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1858;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta576<F: Float>(t23097: F, t4234: F, t776: F, t815: F, t13176: F, t6620: F, t849: F, t25097: F, t81782: F, t81783: F, t1516: F, t81769: F, t23133: F, t4261: F, t25111: F, t25115: F, t87229: F, t23132: F, t4166: F, t25068: F, t2707: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t87316, t87322, t87328, t87330) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1858::<F>(t23097, t4234, t776, t815, t13176, t6620, t849, t25097, t81782, t81783, t1516, t81769);
        let (t87332, t87335, t87338, t87341, t87343) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1859::<F>(t23133, t4261, t25111, t81782, t81783, t25115, t87229, t23132, t4166, t849, t25068, t2707);
    (t87316, t87322, t87328, t87330, t87332, t87335, t87338, t87341, t87343)
}
