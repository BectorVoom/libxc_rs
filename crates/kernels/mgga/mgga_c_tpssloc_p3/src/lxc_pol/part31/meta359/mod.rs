//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1276;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1277;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta359<F: Float>(t1788: F, t2225: F, t2221: F, t225: F, t5213: F, t5211: F, t1372: F, t1824: F, t5286: F, t562: F, t12248: F, t68: F, t544: F, t5230: F) -> (F, F, F, F, F, F, F, F) {
        let (t15982, t15984, t16022, t16030, t16036, t16040, t16046) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1276::<F>(t1788, t2225, t2221, t225, t5213, t5211, t1372, t1824, t5286, t562, t12248, t68);
        let (t16047, t16060) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1277::<F>(t16046, t544, t5230, t68);
    (t15982, t15984, t16022, t16030, t16036, t16040, t16047, t16060)
}
