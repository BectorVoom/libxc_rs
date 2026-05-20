//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2226/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2226<F: Float>(t10189: F, t5842: F, t2986: F, t2990: F, t13847: F, t13861: F, t17841: F, t2987: F, t13784: F, t17178: F, t5836: F, t17161: F) -> (F, F, F, F, F, F, F, F) {
    let t61189 = t10189 * t5842;
    let t61191 = t2986 * t61189 * t2990;
    let t61200 = t2986 * t13847 * t13861;
    let t61210 = t2987 * t17841;
    let t61245 = t2986 * t13784 * t17178;
    let t61250 = t10189 * t5836;
    let t61252 = t2986 * t61250 * t2990;
    let t61258 = t2986 * t13784 * t17161;
    (t61189, t61191, t61200, t61210, t61245, t61250, t61252, t61258)
}
