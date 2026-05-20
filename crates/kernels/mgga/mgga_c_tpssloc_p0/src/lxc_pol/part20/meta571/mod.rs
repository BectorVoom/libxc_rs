//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2134;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta571<F: Float>(t10186: F, t10191: F, t13783: F, t984: F, t10237: F, t2986: F, t10277: F, t343: F, t9288: F, t3014: F, t4509: F, t10273: F, t2960: F) -> (F, F, F, F, F, F, F) {
        let (t42833, t42837, t42839, t42841, t42842, t42846, t42855) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2134::<F>(t10186, t10191, t13783, t984, t10237, t2986, t10277, t343, t9288, t3014, t4509, t10273, t2960);
    (t42833, t42837, t42839, t42841, t42842, t42846, t42855)
}
