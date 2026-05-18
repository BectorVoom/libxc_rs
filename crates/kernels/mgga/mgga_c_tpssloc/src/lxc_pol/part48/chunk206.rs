//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 206/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk206<F: Float>(t241: F, t835: F, t244: F, t248: F, t238: F, t234: F, t236: F, t240: F, t812: F, t200: F, t243: F, t67: F) -> (F, F, F, F, F, F, F, F) {
    let t836 = t835 * t241;
    let t838 = t836 * t244 * t248;
    let t840 = F::new(7.0) / F::new(4608.0) * t238 * t838;
    let t841 = t234 * t236;
    let t842 = t841 * t240;
    let t843 = t812 * t842;
    let t845 = F::new(1.0) / t243 / t200;
    let t847 = t241 * t845 * t67;
    (t836, t838, t840, t841, t842, t843, t845, t847)
}
