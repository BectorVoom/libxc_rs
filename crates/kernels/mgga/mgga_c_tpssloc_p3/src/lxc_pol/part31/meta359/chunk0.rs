//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1276/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1276<F: Float>(t1788: F, t2225: F, t2221: F, t225: F, t5213: F, t5211: F, t1372: F, t1824: F, t5286: F, t562: F, t12248: F, t68: F) -> (F, F, F, F, F, F, F) {
    let t15982 = t2225 * t1788;
    let t15984 = t2221 * t1788;
    let t16022 = t5213 * t225;
    let t16030 = t5211 * t225;
    let t16036 = t1372 * t1824;
    let t16040 = t562 * t5286;
    let t16046 = t68 * t12248;
    (t15982, t15984, t16022, t16030, t16036, t16040, t16046)
}
