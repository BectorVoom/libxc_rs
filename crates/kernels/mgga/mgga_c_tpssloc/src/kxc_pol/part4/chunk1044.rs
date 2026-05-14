//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1044/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1044<F: Float>(t1196: F, t16558: F, t974: F, t1215: F, t1653: F, t15659: F, t3578: F, t1177: F, t18221: F, t18237: F, t1735: F, t4724: F, t11668: F, t18232: F, t3440: F, t1017: F, t6163: F) -> (F, F, F, F, F, F, F) {
    let t18996 = t1196 * t16558;
    let t18997 = t974 * t18996;
    let t19000 = t1653 * t1215;
    let t19001 = t15659 * t19000;
    let t19002 = t3578 * t19001;
    let t19005 = t1177 * t18221;
    let t19010 = t1177 * t18237;
    let t19015 = t1735 * t4724;
    let t19016 = t11668 * t19015;
    let t19019 = t3440 * t18232;
    let t19024 = t6163 * t1017;
    (t18997, t19002, t19005, t19010, t19016, t19019, t19024)
}
