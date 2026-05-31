//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2429/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2429<F: Float>(t13748: F, t2960: F, t1025: F, t10884: F, t10937: F, t14041: F, t1539: F, t2780: F, t3070: F, t3071: F, t42483: F, t42552: F, t42557: F, t42578: F, t42582: F, t4650: F, t49658: F, t49662: F, t49666: F, t49678: F, t49682: F) -> F {
    let t49684 = t2960 * t13748;
    let t49688 = -F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t49658 - t49662 - t10937 * t14041 / F::cast_from(288.0_f64) + t49666 / F::cast_from(2304.0_f64) + t42483 * t3071 * t1539 * t10884 / F::cast_from(4608.0_f64) + t3070 * t3071 * t4650 * t2780 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t42552 + F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t42557 + F::cast_from(19.0_f64) / F::cast_from(576.0_f64) * t49678 * t1025 + t49682 / F::cast_from(1152.0_f64) + t49684 / F::cast_from(27.0_f64) - t42578 / F::cast_from(144.0_f64) - t42582 / F::cast_from(144.0_f64);
    t49688
}
