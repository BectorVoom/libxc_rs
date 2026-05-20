//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1329/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1329<F: Float>(t72: F, t79: F, t9342: F, t531: F, t6995: F, t1983: F, t22596: F, t12012: F, t1390: F, t6878: F, t22574: F, t39367: F, t8643: F) -> (F, F, F, F) {
    let t83846 = t72 * t79 * t9342;
    let t83859 = t531 * t6995;
    let t83862 = F::new(18.0) * t1983 * t83859 * t22596;
    let t83863 = t1390 * t12012;
    let t83866 = F::new(3.0) * t1983 * t6878 * t83863;
    let t83869 = F::new(9.0) * t22574 * t8643 * t39367;
    (t83846, t83862, t83866, t83869)
}
