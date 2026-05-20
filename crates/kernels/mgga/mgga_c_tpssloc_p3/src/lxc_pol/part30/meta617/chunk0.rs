//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2016/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2016<F: Float>(t6695: F, t82632: F, t1920: F, t2966: F, t6699: F, t6707: F, t11094: F, t6818: F, t1958: F, t43637: F, t1081: F, t2752: F) -> (F, F, F, F, F, F) {
    let t83368 = t82632 * t6695;
    let t83444 = t1920 * t2966 * t6699;
    let t83459 = t82632 * t6707;
    let t83472 = t6818 * t11094;
    let t83479 = t1958 * t43637;
    let t83555 = t2752 * t1081;
    (t83368, t83444, t83459, t83472, t83479, t83555)
}
