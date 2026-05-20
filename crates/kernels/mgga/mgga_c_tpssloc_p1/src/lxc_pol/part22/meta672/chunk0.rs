//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2227/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2227<F: Float>(t17152: F, t2986: F, t48213: F, t17863: F, t42837: F, t10186: F, t17808: F, t10236: F, t17635: F, t13835: F, t13847: F, t13839: F, t48279: F) -> (F, F, F, F, F, F) {
    let t61261 = t2986 * t48213 * t17152;
    let t61264 = t2986 * t42837 * t17863;
    let t61273 = t10186 * t17808;
    let t61279 = t10236 * t17635;
    let t61288 = t2986 * t13847 * t13835;
    let t61291 = t2986 * t48279 * t13839;
    (t61261, t61264, t61273, t61279, t61288, t61291)
}
