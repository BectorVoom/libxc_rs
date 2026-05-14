//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1322/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1322<F: Float>(t1689: F, t65097: F, t19305: F, t5522: F, t19585: F, t5706: F, t12836: F, t19620: F, t7310: F, t1760: F, t19571: F, t5757: F, t1777: F, t3245: F, t6245: F, t18289: F, t19604: F) -> (F, F, F, F, F, F, F) {
    let t65099 = 4.0 * t65097 * t1689;
    let t65101 = 4.0 * t19305 * t5522;
    let t65106 = 2.0 * t5706 * t19585;
    let t65109 = 12.0 * t19620 * t7310 * t12836;
    let t65115 = 2.0 * t1760 * t19571 * t5757;
    let t65122 = t3245 * t1777;
    let t65125 = 6.0 * t1760 * t65122 * t6245;
    let t65128 = 6.0 * t1760 * t18289 * t19604;
    (t65099, t65101, t65106, t65109, t65115, t65125, t65128)
}
