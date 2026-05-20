//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1536;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1537;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta403<F: Float>(t248: F, t3051: F, t5681: F, t1041: F, t1616: F, t4338: F, t10408: F, t1409: F, t14219: F, t14218: F, t3071: F, t2940: F, t5804: F, t14459: F, t4496: F, t959: F, t17194: F, t17197: F, t17209: F, t17301: F, t17303: F, t17306: F, t17372: F, t17374: F, t17377: F, t17379: F, t17425: F, t17427: F, t17561: F, t17563: F, t17568: F, t300: F, t5769: F, t961: F, t2904: F, t5790: F, t952: F, t14473: F, t1589: F, t4483: F, t4493: F, t4489: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17906, t17907, t17920, t17925, t17929) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1536::<F>(t248, t3051, t5681, t1041, t1616, t4338, t10408, t1409, t14219, t14218, t3071, t2940, t5804);
        let (t17932, t17933) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1537::<F>(t14459, t4496, t959, t17194, t17197, t17209, t17301, t17303, t17306, t17372, t17374, t17377, t17379, t17425, t17427, t17561, t17563, t17568, t17929);
        let (t17936, t17940, t17942, t17944, t17946) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1538::<F>(t300, t5769, t961, t2904, t5790, t952, t959, t14473, t1589, t4483, t4493, t4489);
    (t17906, t17907, t17920, t17925, t17929, t17932, t17933, t17936, t17940, t17942, t17944, t17946)
}
