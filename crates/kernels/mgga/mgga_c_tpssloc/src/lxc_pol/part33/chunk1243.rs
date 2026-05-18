//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1243/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1243<F: Float>(t25721: F, t6743: F, t210: F, t23599: F, t23632: F, t1958: F, t43637: F, t38: F, t9287: F, t835: F, t39063: F, t6489: F) -> (F, F, F, F, F, F) {
    let t83240 = t6743 * t25721;
    let t83244 = t23599 * t210;
    let t83245 = t83244 * t23632;
    let t83479 = t1958 * t43637;
    let t83796 = t38 * t9287;
    let t83803 = F::new(1232.0) / F::new(27.0) * t835;
    let t83830 = t39063 * t6489;
    (t83240, t83245, t83479, t83796, t83803, t83830)
}
