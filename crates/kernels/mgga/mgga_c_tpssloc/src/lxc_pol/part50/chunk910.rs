//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 910/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk910<F: Float>(t1527: F, t857: F, t776: F, t23270: F, t22986: F, t225: F, t258: F, t4265: F, t214: F, t1880: F, t1484: F, t22690: F, t841: F) -> (F, F, F, F) {
    let t25053 = t857 * t1527;
    let t25054 = t25053 * t776;
    let t25055 = t23270 * t25054;
    let t25056 = t22986 * t25055;
    let t25059 = t4265 * t225 * t258;
    let t25060 = t214 * t25059;
    let t25061 = t1880 * t25060;
    let t25064 = t22690 * t841 * t1484;
    (t25054, t25056, t25061, t25064)
}
