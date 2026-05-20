//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1628;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta444<F: Float>(t23253: F, t6562: F, t225: F, t258: F, t2710: F, t214: F, t1880: F, t1883: F, t23012: F, t23237: F, t6572: F, t213: F, t252: F) -> (F, F, F, F, F, F, F, F) {
        let (t23254, t23257, t23258, t23259, t23261, t23265, t23266, t23270) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1628::<F>(t23253, t6562, t225, t258, t2710, t214, t1880, t1883, t23012, t23237, t6572, t213, t252);
    (t23254, t23257, t23258, t23259, t23261, t23265, t23266, t23270)
}
