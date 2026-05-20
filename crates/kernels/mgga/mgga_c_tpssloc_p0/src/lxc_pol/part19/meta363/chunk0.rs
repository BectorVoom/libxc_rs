//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1323/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1323<F: Float>(t1012: F, t1015: F, t1017: F, t10444: F, t41620: F, t41622: F, t41625: F, t41627: F, t41635: F, t41639: F, t41722: F, t41726: F, t41728: F, t41732: F, t41737: F) -> (F, F) {
    let t42658 = t1012 * t1015 * t10444 * t1017;
    let t42661 = t41620 + t41622 + t41625 + t41627 + t41635 + t41639 - t41722 - t41726 + t41728 + t41732 + t41737;
    (t42658, t42661)
}
