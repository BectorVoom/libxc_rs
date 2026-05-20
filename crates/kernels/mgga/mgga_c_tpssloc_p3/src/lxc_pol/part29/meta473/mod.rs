//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta473<F: Float>(t1268: F, t22479: F, t12461: F, t3698: F, t2019: F, t1983: F, t12521: F, t1873: F, t12524: F, t7015: F, t3938: F, t6534: F) -> (F, F, F, F, F, F, F) {
        let (t23854, t23857, t23858, t23860, t23886, t23888, t23890) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1809::<F>(t1268, t22479, t12461, t3698, t2019, t1983, t12521, t1873, t12524, t7015, t3938, t6534);
    (t23854, t23857, t23858, t23860, t23886, t23888, t23890)
}
