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
    let t49688 = -F::new(2.0) / F::new(81.0) * t49658 - t49662 - t10937 * t14041 / F::new(288.0) + t49666 / F::new(2304.0) + t42483 * t3071 * t1539 * t10884 / F::new(4608.0) + t3070 * t3071 * t4650 * t2780 / F::new(1536.0) + F::new(5.0) / F::new(1296.0) * t42552 + F::new(11.0) / F::new(324.0) * t42557 + F::new(19.0) / F::new(576.0) * t49678 * t1025 + t49682 / F::new(1152.0) + t49684 / F::new(27.0) - t42578 / F::new(144.0) - t42582 / F::new(144.0);
    t49688
}
