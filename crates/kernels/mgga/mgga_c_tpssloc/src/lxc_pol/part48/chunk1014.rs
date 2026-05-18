//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1014/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1014<F: Float>(t24462: F, t6534: F, t131: F, t2108: F, t39063: F, t8662: F, t31867: F, t9239: F, t2240: F, t24503: F, t8301: F, t39049: F) -> (F, F, F, F, F, F) {
    let t116008 = F::new(27.0) * t24462 * t6534;
    let t116065 = t2108 * t131;
    let t116075 = t39063 * t8662;
    let t116082 = t9239 * t31867;
    let t116088 = t2240 * t8301 * t24503;
    let t116096 = t39049 * t8662;
    (t116008, t116065, t116075, t116082, t116088, t116096)
}
