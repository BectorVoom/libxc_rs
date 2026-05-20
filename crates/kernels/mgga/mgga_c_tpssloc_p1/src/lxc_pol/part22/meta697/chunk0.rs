//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2279/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2279<F: Float>(t18392: F, t3490: F, t1227: F, t18241: F, t248: F, t3521: F, t19040: F, t15734: F, t5024: F, t11818: F, t3515: F, t6230: F) -> (F, F, F, F, F) {
    let t65613 = t3490 * t18392;
    let t65617 = t1227 * t248 * t3521 * t18241;
    let t65619 = t3490 * t19040;
    let t65628 = t5024 * t15734;
    let t65632 = t3515 * t248 * t11818 * t6230;
    (t65613, t65617, t65619, t65628, t65632)
}
