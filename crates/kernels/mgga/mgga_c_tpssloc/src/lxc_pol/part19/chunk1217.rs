//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1217/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1217<F: Float>(t41620: F, t41622: F, t41625: F, t41627: F, t41635: F, t41639: F, t41722: F, t41726: F, t41728: F, t41732: F, t41737: F, t10526: F, t2940: F, t10623: F, t2948: F, t10709: F) -> (F, F, F, F) {
    let t42661 = t41620 + t41622 + t41625 + t41627 + t41635 + t41639 - t41722 - t41726 + t41728 + t41732 + t41737;
    let t42663 = 0.4155806185363551302e3 * t2940 * t10526;
    let t42665 = 0.35089341735807877242e1 * t10623 * t2948;
    let t42667 = 0.14035736694323150897e2 * t2940 * t10709;
    (t42661, t42663, t42665, t42667)
}
