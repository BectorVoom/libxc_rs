//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 769/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk769<F: Float>(t491: F, t4940: F, t1235: F, t1720: F, t1721: F, t225: F, t1190: F, t1751: F, t1090: F, t1735: F, t3578: F, t1216: F, t1653: F, t1222: F, t1731: F, t1744: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4941 = t4940 * t491;
    let t4943 = t1720 * t1235;
    let t4945 = t1721 * t225;
    let t4947 = t1190 * t1751;
    let t4949 = t1735 * t1090;
    let t4950 = t3578 * t4949;
    let t4953 = t1653 * t1216;
    let t4954 = t3578 * t4953;
    let t4957 = t1731 * t1222;
    let t4959 = t1744 * t1222;
    (t4941, t4943, t4945, t4947, t4949, t4950, t4953, t4954, t4957, t4959)
}
