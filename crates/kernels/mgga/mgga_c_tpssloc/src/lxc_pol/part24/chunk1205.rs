//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1205/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1205<F: Float>(t81920: F, t23109: F, t23110: F, t232: F, t81915: F, t23116: F, t838: F, t2693: F, t6609: F, t10041: F, t6581: F, t213: F, t6589: F, t9223: F, t6593: F, t23062: F, t23066: F) -> (F, F, F, F, F, F, F) {
    let t81921 = 595.0 / 5184.0 * t81920;
    let t81924 = t23109 * t23110 * t81915 * t232;
    let t81926 = t23116 * t838;
    let t81928 = t6609 * t2693;
    let t81930 = t6581 * t10041;
    let t81933 = t9223 * t6589 * t213;
    let t81934 = t81933 * t6593;
    let t81936 = t23062 * t23066;
    (t81921, t81924, t81926, t81928, t81930, t81934, t81936)
}
