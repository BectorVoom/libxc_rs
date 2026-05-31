//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2036/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2036<F: Float>(t81920: F, t23109: F, t23110: F, t232: F, t81915: F, t23116: F, t838: F, t2693: F, t6609: F, t213: F, t6589: F, t9223: F) -> (F, F, F, F, F) {
    let t81921 = F::cast_from(595.0_f64) / F::cast_from(5184.0_f64) * t81920;
    let t81924 = t23109 * t23110 * t81915 * t232;
    let t81926 = t23116 * t838;
    let t81928 = t6609 * t2693;
    let t81933 = t9223 * t6589 * t213;
    (t81921, t81924, t81926, t81928, t81933)
}
