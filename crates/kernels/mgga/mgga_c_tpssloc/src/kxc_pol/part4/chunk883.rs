//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 883/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk883<F: Float>(t1215: F, t1734: F, t1089: F, t475: F, t1744: F, t3540: F, t1731: F, t1222: F, t4961: F, t1706: F, t3545: F, t11818: F, t1735: F, t248: F, t1213: F, t11789: F, t1653: F) -> (F, F, F, F, F, F, F, F) {
    let t15700 = t1734 * t1215;
    let t15701 = t475 * t1089;
    let t15717 = t1744 * t3540;
    let t15719 = t1731 * t3540;
    let t15722 = t4961 * t1222 / 432.0;
    let t15727 = t1706 * t3545;
    let t15730 = t248 * t11818 * t1735;
    let t15731 = t1213 * t15730;
    let t15734 = t248 * t11789 * t1653;
    (t15700, t15701, t15717, t15719, t15722, t15727, t15731, t15734)
}
