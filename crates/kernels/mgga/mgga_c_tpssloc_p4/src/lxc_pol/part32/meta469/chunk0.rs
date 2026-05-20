//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1759/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1759<F: Float>(t461: F, t52: F, t1009: F, t7324: F, t1210: F, t7330: F, t3502: F, t3504: F, t3500: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t24719 = t52 * t461;
    let t24720 = t24719 * t1009;
    let t24721 = t7324 * t24720;
    let t24722 = t1210 * t7330;
    let t24723 = t24721 * t24722;
    let t24727 = t3502 * sigma2;
    let t24728 = t24727 * t3504;
    let t24729 = t3500 * t24728;
    (t24719, t24720, t24721, t24722, t24723, t24727, t24728, t24729)
}
