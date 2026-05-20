//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3129/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3129<F: Float>(t16558: F, t3450: F, t11588: F, t6138: F, t3447: F, t3451: F, t4904: F, t52036: F, t15313: F, t15338: F, t18523: F, t3448: F) -> (F, F, F, F, F) {
    let t64756 = t3450 * t16558;
    let t64763 = t11588 * t6138;
    let t64765 = t3447 * t64763 * t3451;
    let t64770 = t3447 * t52036 * t4904;
    let t64773 = t3447 * t15338 * t15313;
    let t64775 = t3448 * t18523;
    (t64756, t64765, t64770, t64773, t64775)
}
