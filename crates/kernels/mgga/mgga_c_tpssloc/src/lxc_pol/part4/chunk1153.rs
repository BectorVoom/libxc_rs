//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1153/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1153<F: Float>(t15453: F, t17686: F, t4582: F, t17635: F, t4972: F, t1090: F, t6230: F, t3578: F, t6219: F, t4997: F, t5002: F, t11784: F, t248: F, t5971: F) -> (F, F, F, F, F, F) {
    let t18954 = t15453 * t17686;
    let t18955 = t4582 * t18954;
    let t18958 = t4972 * t17635;
    let t18959 = t4582 * t18958;
    let t18964 = t6230 * t1090;
    let t18965 = t3578 * t18964;
    let t18968 = t6219 * t1090;
    let t18969 = t3578 * t18968;
    let t18972 = t5002 * t4997;
    let t18975 = t248 * t11784 * t5971;
    (t18955, t18959, t18965, t18969, t18972, t18975)
}
