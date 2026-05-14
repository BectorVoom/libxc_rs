//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 789/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk789<F: Float>(t1055: F, t11084: F, t10160: F, t10167: F, t10170: F, t10182: F, t1052: F, t1066: F, t11008: F, t11010: F, t11013: F, t11016: F, t11018: F, t3026: F, t3169: F, t3176: F, t3207: F, t388: F) -> (F,) {
    let t11085 = t1055 * t11084;
    let t11087 = -6.0 * t10160 * t1066 - 6.0 * t10167 * t1052 - 3.0 * t10170 * t1066 + 6.0 * t10182 * t1052 - t1052 * t11085 - 3.0 * t1066 * t11010 + t11008 * t388 + 3.0 * t11013 * t388 + t11016 * t388 + 3.0 * t11018 * t388 + 6.0 * t3026 * t3176 - 3.0 * t3026 * t3207 + 6.0 * t3169 * t3176 - 3.0 * t3169 * t3207;
    (t11087,)
}
