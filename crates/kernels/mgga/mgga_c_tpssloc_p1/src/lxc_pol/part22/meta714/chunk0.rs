//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2317/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2317<F: Float>(t21061: F, t225: F, t21036: F, t20856: F, t252: F, t1519: F, t5584: F, t20852: F, t13176: F, t13433: F, t13453: F, t16673: F, t16756: F, t16758: F, t16762: F, t16817: F, t16825: F, t16830: F, t16935: F, t17034: F, t21025: F, t4166: F, t4182: F, t4281: F, t4296: F, t5612: F, t5645: F, t5651: F, t58313: F, t812: F) -> (F, F, F, F, F, F) {
    let t67339 = t21061 * t225;
    let t67344 = t21036 * t225;
    let t67350 = t252 * t20856;
    let t67358 = t1519 * t5584;
    let t67392 = t252 * t20852;
    let t67403 = -F::new(3.0) * t13433 * t5612 * t812 + F::new(12.0) * t16758 * t16935 * t4281 + F::new(2.0) * t4182 * t4281 * t67392 + F::new(6.0) * t13176 * t5645 - F::new(3.0) * t13176 * t5651 + F::new(6.0) * t13453 * t21025 - F::new(3.0) * t16673 * t4296 - F::new(3.0) * t16756 * t4166 - F::new(6.0) * t16762 * t16830 - F::new(18.0) * t16817 * t58313 + F::new(18.0) * t16825 * t17034;
    (t67339, t67344, t67350, t67358, t67392, t67403)
}
