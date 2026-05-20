//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1362/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1362<F: Float>(t1005: F, t3082: F, t1036: F, t3094: F, t3089: F, t248: F, t2780: F, t3051: F, t1041: F, t121: F, t3061: F, t2771: F) -> (F, F, F, F, F, F) {
    let t10436 = t1005 * t3082;
    let t10441 = t3094 * t1036;
    let t10449 = t3089 * t1036;
    let t10454 = t248 * t3051 * t2780;
    let t10455 = t1041 * t10454;
    let t10457 = t121 * t3061;
    let t10459 = t248 * t10457 * t2771;
    (t10436, t10441, t10449, t10454, t10455, t10459)
}
