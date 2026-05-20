//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta383 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1456;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1457;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1458;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta383<F: Float>(t120: F, t5584: F, t16816: F, t4180: F, t4182: F, t5593: F, t9638: F, t5527: F, t776: F, t820: F, t9607: F, t16753: F, t819: F, t13087: F, t13182: F, t13190: F, t13202: F, t13208: F, t13234: F, t13237: F, t13262: F, t16836: F, t2618: F, t4172: F, t4178: F, t4184: F, t4257: F, t5587: F, t5614: F, t5619: F, t817: F, t843: F, t9602: F, t9672: F, t9967: F) -> (F, F, F, F, F, F, F) {
        let t16839 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1456::<F>(t120, t5584);
        let (t16841, t16845, t16848, t16851, t16853, t16859) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1457::<F>(t16816, t16839, t4180, t4182, t5593, t9638, t5527, t776, t820, t9607, t16753, t819);
        let t16869 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1458::<F>(t13087, t13182, t13190, t13202, t13208, t13234, t13237, t13262, t16836, t16841, t16845, t16848, t16853, t16859, t2618, t4172, t4178, t4184, t4257, t5587, t5614, t5619, t817, t843, t9602, t9672, t9967);
    (t16839, t16841, t16845, t16851, t16853, t16859, t16869)
}
