//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 586/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk586<F: Float>(t1174: F, t5040: F, t1196: F, t3966: F, t974: F, t1198: F, t1213: F, t1218: F, t1227: F, t1232: F, t1748: F, t3490: F, t3524: F, t3542: F, t3543: F, t3547: F, t3549: F, t3573: F, t4889: F, t5014: F, t5019: F, t5024: F, t5030: F, t5033: F, t5036: F) -> (F,) {
    let t5041 = t1174 * t5040;
    let t5045 = t1196 * t3966;
    let t5046 = t974 * t5045;
    let t5051 = t1213 * t5014 / 3072.0 - t5019 * t1218 / 576.0 + t5024 * t1232 / 864.0 - t3490 * t1748 / 4608.0 - t1227 * t5030 / 4608.0 + t1174 * t5033 / 216.0 - t5036 / 108.0 - t3524 / 6912.0 + t3573 / 4608.0 - t5041 / 864.0 + t4889 * t1198 / 108.0 - t1174 * t5046 / 288.0 - t3549 / 864.0 - t3542 + t3543 / 4608.0 - t3547;
    (t5051,)
}
