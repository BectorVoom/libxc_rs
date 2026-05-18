//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1313/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1313<F: Float>(t17974: F, t3689: F, t10623: F, t5559: F, t1385: F, t61086: F, t17946: F, t3622: F, t10632: F, t5547: F, t10674: F, t17960: F, t3667: F) -> (F, F, F, F, F, F, F) {
    let t63960 = t17974 * t3689;
    let t63962 = t5559 * t10623;
    let t63964 = t61086 * t1385;
    let t63966 = t17946 * t3622;
    let t63968 = t5547 * t10632;
    let t63971 = t5559 * t10674;
    let t63973 = t17960 * t3667;
    (t63960, t63962, t63964, t63966, t63968, t63971, t63973)
}
