//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1197/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1197<F: Float>(t19676: F, t19679: F, t19688: F, t19699: F, t225: F, t1819: F, t68: F, t1995: F, t6330: F, t1307: F, t5187: F, t5279: F) -> (F, F, F, F) {
    let t19702 = (t19676 + t19679 + t19688 + t19699) * t225;
    let t19708 = t1819 * t68;
    let t19715 = t1995 * t6330;
    let t19716 = t19715 * t1307;
    let t19719 = t5279 * t5187;
    (t19702, t19708, t19716, t19719)
}
