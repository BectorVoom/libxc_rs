//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1266/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1266<F: Float>(t1098: F, t21988: F, t21938: F, t3400: F, t19080: F, t4997: F, t19047: F, t19040: F, t5005: F, t19026: F, t18975: F, t11719: F, t22307: F, t248: F, t3570: F) -> (F, F, F, F, F, F, F, F) {
    let t71877 = t21988 * t1098;
    let t72062 = t3400 * t21938;
    let t72161 = t19080 * t4997;
    let t72181 = t19047 * t4997;
    let t72183 = t5005 * t19040;
    let t72223 = t19026 * t4997;
    let t72225 = t5005 * t18975;
    let t72229 = t11719 * t248 * t3570 * t22307;
    (t71877, t72062, t72161, t72181, t72183, t72223, t72225, t72229)
}
