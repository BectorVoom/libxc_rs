//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta425<F: Float>(t1557: F, t2793: F, t2842: F, t4434: F, t931: F, t10740: F, t10765: F, t14376: F, t14378: F, t14381: F, t14384: F, t14387: F, t14391: F, t14394: F, t14398: F, t14419: F, t2861: F, t311: F, t4416: F, t4438: F) -> (F, F, F, F) {
        let (t14422, t14424, t14425, t14428) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1841::<F>(t1557, t2793, t2842, t4434, t931, t10740, t10765, t14376, t14378, t14381, t14384, t14387, t14391, t14394, t14398, t14419, t2861, t311, t4416, t4438);
    (t14422, t14424, t14425, t14428)
}
