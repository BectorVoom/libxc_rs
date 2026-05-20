//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1716;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta414<F: Float>(t1119: F, t18686: F, t14845: F, t1671: F, t4740: F, t4782: F, t11424: F, t5989: F, t3259: F, t6021: F, t11136: F, t11137: F, t14702: F, t14922: F, t14923: F, t14924: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18227: F, t18229: F, t18234: F, t18239: F, t18243: F, t449: F, t11247: F, t14721: F, t14723: F, t14724: F) -> (F, F, F, F, F, F, F, F) {
        let (t18688, t18690, t18692, t18694, t18696, t18710) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1716::<F>(t1119, t18686, t14845, t1671, t4740, t4782, t11424, t5989, t3259, t6021, t11136, t11137, t14702, t14922, t14923, t14924, t18203, t18208, t18213, t18217, t18219, t18223, t18227, t18229, t18234, t18239, t18243);
        let (t18711, t18730) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1717::<F>(t18710, t449, t11137, t11247, t14702, t14721, t14723, t14724, t18203, t18208, t18213, t18217, t18219, t18223, t18227, t18229, t18234, t18239, t18243);
    (t18688, t18690, t18692, t18694, t18696, t18710, t18711, t18730)
}
