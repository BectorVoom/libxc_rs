//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1064/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1064<F: Float>(t1646: F, t9994: F, t10137: F, t4405: F, t1206: F, t4408: F, t762: F, t1629: F, t3234: F, t10161: F, t10166: F, t1213: F, t1244: F, t12996: F, t13000: F, t13004: F, t13006: F, t13009: F, t13013: F, t13015: F, t3244: F, t4413: F) -> (F,) {
    let t13018 = t9994 * t1646;
    let t13021 = 7.0 / 24.0 * t10137 * t4405;
    let t13023 = t762 * t4408 * t1206;
    let t13027 = t762 * t1629 * t3234;
    let t13030 = -t1213 * t12996 / 48.0 + t4413 * t13000 / 1536.0 - t13004 + t13006 - 35.0 / 108.0 * t10161 - t10166 - t4413 * t13009 / 384.0 + t13013 - t1244 * t13015 / 768.0 - 119.0 / 3456.0 * t13018 - t13021 + t3244 * t13023 / 8.0 + t3244 * t13027 / 16.0;
    (t13030,)
}
