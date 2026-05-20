//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2331/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2331<F: Float>(t3866: F, t6427: F, t6431: F, t19735: F, t5248: F, t5249: F, t16242: F, t3805: F, t6394: F, t120: F, t6414: F) -> (F, F, F, F, F) {
    let t19940 = t3866 * t6427;
    let t19942 = t3866 * t6431;
    let t19945 = t5248 * t5249 * t19735;
    let t19951 = t3805 * t16242 * t6394;
    let t19956 = t120 * t6414;
    (t19940, t19942, t19945, t19951, t19956)
}
