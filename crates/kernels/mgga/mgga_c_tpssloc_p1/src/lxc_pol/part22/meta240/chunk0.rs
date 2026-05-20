//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1327/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1327<F: Float>(t252: F, t9971: F, t856: F, t68: F, t261: F, t2751: F, t1053: F, t1887: F, t337: F, t615: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10080 = t9971 * t252;
    let t10108 = t856 * t856;
    let t10109 = F::new(1.0) / t10108;
    let t10110 = t68 * t10109;
    let t10143 = F::new(1.0) / t2751 / t261;
    let t10163 = t1053 * t1053;
    let t10164 = F::new(1.0) / t10163;
    let t10165 = t68 * t10164;
    let t10186 = t615 * t337 * t1887;
    (t10080, t10108, t10109, t10110, t10143, t10163, t10164, t10165, t10186)
}
