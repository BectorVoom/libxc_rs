//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 917/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk917<F: Float>(t120: F, t5544: F, t2645: F, t829: F, t16839: F, t2647: F, t13177: F, t13251: F, t13260: F, t13275: F, t13277: F, t13280: F, t13287: F, t13320: F, t13330: F, t1512: F, t16872: F, t16877: F, t16879: F, t16888: F, t16893: F, t16898: F, t2643: F, t4167: F, t4178: F, t4191: F, t4236: F, t4240: F, t4250: F, t831: F) -> (F,) {
    let t16901 = t120 * t5544;
    let t16903 = t2645 * t16901 * t829;
    let t16907 = t2645 * t16839 * t2647;
    let t16910 = -t4167 * t4236 / 1536.0 - t16872 * t831 / 3072.0 - t13177 * t1512 / 1536.0 + 7.0 / 2304.0 * t16877 - 7.0 / 2304.0 * t16879 - t13260 + t13275 + t13277 + t13280 - t13287 + t13251 * t4191 / 384.0 - t13251 * t4240 / 1536.0 + t13251 * t4250 / 384.0 - 5.0 / 384.0 * t2643 * t16888 + t4178 * t16893 / 1536.0 - 5.0 / 768.0 * t2643 * t16898 + t2643 * t16903 / 768.0 + t13320 - t13330 + t2643 * t16907 / 768.0;
    (t16910,)
}
