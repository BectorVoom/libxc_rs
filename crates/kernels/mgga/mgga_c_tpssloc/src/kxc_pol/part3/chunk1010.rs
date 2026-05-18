//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1010/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1010<F: Float>(t12850: F, t12860: F, t12861: F, t12889: F, t12891: F, t12894: F, t12906: F, t12910: F, t9457: F, t9462: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F) -> F {
    let t13093 = t12850 - t9457 + t9462 - t12860 + t12861 - t9469 + t12889 + t12891 + t12894 + t9476 + t9484 - t9496 - t9715 - t12906 + t12910;
    t13093
}
