//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 994/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk994<F: Float>(t3590: F, t72: F, t732: F, t2222: F, t3560: F, t1289: F, t724: F, t581: F, t3564: F, t8212: F, t8218: F, t10353: F, t190: F) -> (F, F, F, F, F, F) {
    let t10684 = t3590 * t72;
    let t10686 = F::cast_from(0.36622894612013090108e-3_f64) * t10684 * t732;
    let t10687 = t3560 * t2222;
    let t10688 = F::cast_from(0.24415263074675393405e-3_f64) * t10687;
    let t10689 = t724 * t1289;
    let t10690 = t10689 * t581;
    let t10692 = F::new(24.0) * t3564 * t10690;
    let t10693 = F::cast_from(0.34631718211362927518e2_f64) * t8212;
    let t10694 = F::cast_from(0.23392894490538584828e1_f64) * t8218;
    let t10695 = t190 * t10353;
    (t10686, t10688, t10692, t10693, t10694, t10695)
}
