//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 705/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk705<F: Float>(t1054: F, t883: F, t1065: F, t607: F, t23329: F, t381: F, t6733: F, t6691: F, t1955: F, t3175: F, t10165: F, t6686: F, t6712: F, t225: F, t3166: F, t387: F) -> (F, F, F, F, F) {
    let t23330 = t1054 * t883;
    let t23331 = t607 * t1065;
    let t23332 = t23330 * t23331;
    let t23333 = t23329 * t23332;
    let t23336 = t6733 * t381;
    let t23337 = t23336 * t6691;
    let t23340 = t1955 * t3175;
    let t23341 = t10165 * t23340;
    let t23346 = t6712 * t6686;
    let t23353 = t3166 * t225 * t387;
    (t23333, t23337, t23341, t23346, t23353)
}
