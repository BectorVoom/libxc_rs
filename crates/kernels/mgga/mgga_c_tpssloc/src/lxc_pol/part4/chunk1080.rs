//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1080/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1080<F: Float>(t248: F, t3051: F, t5681: F, t1041: F, t1616: F, t4338: F, t10408: F, t1409: F, t14219: F, t14218: F, t3071: F, t2940: F, t5804: F) -> (F, F, F, F) {
    let t17906 = t248 * t3051 * t5681;
    let t17907 = t1041 * t17906;
    let t17919 = t1616 * t4338;
    let t17920 = t10408 * t17919;
    let t17923 = t14219 * t1409;
    let t17924 = t14218 * t17923;
    let t17925 = t3071 * t17924;
    let t17929 = F::new(0.11696447245269292414e1) * t2940 * t5804;
    (t17907, t17920, t17925, t17929)
}
