//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1510/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1510<F: Float>(t10431: F, t10513: F, t10929: F, t11005: F, t349: F, t225: F, t3167: F, t3166: F, t990: F, t10358: F, t381: F, t1049: F, t3020: F) -> (F, F, F, F, F, F) {
    let t11007 = t10431 + t10513 + t10929 + t11005;
    let t11008 = t349 * t11007;
    let t11010 = t3167 * t225;
    let t11013 = t990 * t3166;
    let t11016 = t10358 * t381;
    let t11018 = t3020 * t1049;
    (t11007, t11008, t11010, t11013, t11016, t11018)
}
