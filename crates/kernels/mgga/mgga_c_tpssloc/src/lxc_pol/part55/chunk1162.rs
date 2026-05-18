//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1162/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1162<F: Float>(t32578: F, t9231: F, t111: F, t32594: F, t1089: F, t2154: F, t2144: F, t225: F, t461: F, t1240: F, t7391: F, t24574: F, t32516: F) -> (F, F, F, F, F, F) {
    let t117762 = t9231 * t32578;
    let t117773 = t32594 * t111;
    let t117803 = t2154 * t1089;
    let t117809 = t461 * t2144 * t225;
    let t117813 = t1240 * t7391;
    let t117823 = t24574 * t32516;
    (t117762, t117773, t117803, t117809, t117813, t117823)
}
