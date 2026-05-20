//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 905/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk905<F: Float>(t1976: F, t4072: F, t671: F, t7670: F, t191: F, t192: F, t5118: F, t2020: F, t6997: F, t7685: F, t1390: F, t5187: F) -> (F, F, F, F, F, F) {
    let t24980 = t1976 * t4072;
    let t24983 = t7670 * t671;
    let t24987 = t5118 * t191 * t192;
    let t24988 = t24987 * t2020;
    let t24989 = t7685 * t6997;
    let t24990 = t1390 * t5187;
    (t24980, t24983, t24987, t24988, t24989, t24990)
}
