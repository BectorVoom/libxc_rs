//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 427/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk427<F: Float>(t1788: F, t588: F, t592: F, t1831: F, t3866: F, t1835: F, t225: F) -> (F, F, F, F) {
    let t5264 = t588 * t1788;
    let t5266 = t592 * t1788;
    let t5306 = t3866 * t1831;
    let t5321 = t1835 * t225;
    (t5264, t5266, t5306, t5321)
}
