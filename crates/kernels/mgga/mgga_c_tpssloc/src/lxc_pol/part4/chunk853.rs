//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 853/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk853<F: Float>(t2697: F, t4261: F, t820: F, t9645: F, t1484: F, t828: F, t1516: F, t9993: F, t2696: F, t4166: F, t849: F, t13176: F, t842: F, t9601: F, t68: F, t9971: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13345 = 7.0 / 576.0 * t2697 * t4261;
    let t13350 = t9645 * t820;
    let t13351 = t1484 * t828;
    let t13359 = 7.0 / 576.0 * t9993 * t1516;
    let t13360 = t4166 * t2696;
    let t13362 = 7.0 / 576.0 * t13360 * t849;
    let t13365 = t13176 * t842;
    let t13368 = t9601 * t1516;
    let t13396 = t68 * t9971;
    (t13345, t13350, t13351, t13359, t13360, t13362, t13365, t13368, t13396)
}
