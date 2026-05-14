//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1202/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1202<F: Float>(t477: F, t6238: F, t24574: F, t29777: F, t29678: F, t7359: F, t29759: F, t29790: F, t29763: F, t8067: F, t94490: F, t27604: F, t4993: F, t19095: F, t24733: F, t1207: F, t19024: F, t7337: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t103881 = t477 * t6238;
    let t103927 = t24574 * t29777;
    let t103939 = t29678 * t7359;
    let t103943 = t24574 * t29759;
    let t103950 = t24574 * t29790;
    let t103954 = t24574 * t29763;
    let t103959 = t94490 * t8067;
    let t104007 = t27604 * t4993;
    let t104009 = t24733 * t19095;
    let t104012 = t1207 * t7337 * t19024;
    (t103881, t103927, t103939, t103943, t103950, t103954, t103959, t104007, t104009, t104012)
}
