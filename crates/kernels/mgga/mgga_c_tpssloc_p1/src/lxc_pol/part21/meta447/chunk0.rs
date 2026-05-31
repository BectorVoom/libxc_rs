//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1995/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1995<F: Float>(t1089: F, t1215: F, t607: F, t15659: F, t3578: F, t1196: F, t12606: F, t974: F, t3548: F, t4889: F, t14736: F, t3440: F) -> (F, F, F, F, F, F, F) {
    let t15660 = t1215 * t1089;
    let t15661 = t15660 * t607;
    let t15662 = t15659 * t15661;
    let t15663 = t3578 * t15662;
    let t15666 = t1196 * t12606;
    let t15667 = t974 * t15666;
    let t15671 = t4889 * t3548 / F::cast_from(162.0_f64);
    let t15672 = t3440 * t14736;
    (t15661, t15662, t15663, t15666, t15667, t15671, t15672)
}
