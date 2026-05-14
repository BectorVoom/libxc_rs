//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1158/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1158<F: Float>(t21809: F, t3315: F, t21886: F, t3359: F, t1147: F, t21826: F, t1128: F, t21975: F, t1098: F, t21988: F, t21938: F, t3400: F, t19080: F, t4997: F, t19047: F, t19040: F, t5005: F) -> (F, F, F, F, F, F, F, F, F) {
    let t71701 = t21809 * t3315;
    let t71729 = t21886 * t3359;
    let t71860 = t21826 * t1147;
    let t71863 = t21975 * t1128;
    let t71877 = t21988 * t1098;
    let t72062 = t3400 * t21938;
    let t72161 = t19080 * t4997;
    let t72181 = t19047 * t4997;
    let t72183 = t5005 * t19040;
    (t71701, t71729, t71860, t71863, t71877, t72062, t72161, t72181, t72183)
}
