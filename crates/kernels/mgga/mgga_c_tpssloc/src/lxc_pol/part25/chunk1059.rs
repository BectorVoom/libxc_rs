//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1059/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1059<F: Float>(t225: F, t24141: F, t2085: F, t3850: F, t12178: F, t12267: F, t1336: F, t1352: F, t24116: F, t24128: F, t24131: F, t3773: F, t3777: F, t3856: F, t5250: F, t5334: F, t5344: F, t7208: F, t7209: F, t7211: F, t81016: F, t81019: F, t81022: F, t81031: F, t81037: F, t81039: F, t81041: F, t81043: F, t81047: F, t81050: F) -> (F, F) {
    let t84433 = t24141 * t225;
    let t84441 = t2085 * t3850;
    let t84471 = -3.0 * t5344 * t84441 * t1352 + 6.0 * t5334 * t84441 * t5250 + 0.9869604401089358619e-1 * t81016 + 0.9869604401089358619e-1 * t81019 - 0.49348022005446793095e-1 * t81022 - 0.9869604401089358619e-1 * t81031 + 3.0 * t3773 * t7211 - t1336 * t7208 * t12178 - 3.0 * t12267 * t7209 - 3.0 * t3777 * t24131 + 6.0 * t3777 * t24128 - 3.0 * t1336 * t24116 * t3856 - 0.11514538467937585055e0 * t81037 + 0.38381794893125283518e0 * t81039 + 0.11514538467937585055e0 * t81041 - 0.69087230807625510332e0 * t81043 - 0.15626873635058151147e0 * t81047 + 0.49348022005446793095e-1 * t81050;
    (t84433, t84471)
}
