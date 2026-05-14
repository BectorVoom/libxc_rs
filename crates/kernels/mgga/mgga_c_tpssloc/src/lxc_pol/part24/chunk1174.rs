//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1174/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1174<F: Float>(t22649: F, t6883: F, t1372: F, t212: F, t22642: F, t6890: F, t1985: F, t22666: F, t22934: F, t1988: F, t81071: F, t225: F, t22942: F, t22643: F, t22637: F, t81228: F) -> (F, F, F, F, F, F) {
    let t81307 = t6883 * t22649;
    let t81311 = t22642 * t212 * t1372 * t6890;
    let t81315 = t1985 * t22666 * t22934;
    let t81317 = t81071 * t1988;
    let t81318 = 0.27720185200590482541e0 * t81317;
    let t81319 = t22942 * t225;
    let t81326 = t22643 * t225;
    let t81328 = t81228 * t81326 * t22637;
    (t81307, t81311, t81315, t81318, t81319, t81328)
}
