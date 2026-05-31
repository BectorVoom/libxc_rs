//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1850/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1850<F: Float>(t24949: F, t24953: F, t3: F, t112: F, t7415: F, t111: F, t2169: F, t2319: F, t2363: F, t23886: F, t23888: F, t23890: F, t23892: F, t23895: F, t23898: F, t23900: F, t577: F, t671: F, t7423: F) -> (F, F, F, F, F) {
    let t24954 = t24949 + t24953;
    let t24955 = t3 * t24954;
    let t24969 = t7415 * t112;
    let t24972 = t2169 * t111;
    let t24977 = F::cast_from(0.45e1_f64) * t24954 * t577 + F::cast_from(27.0_f64) * t24969 * t671 + F::cast_from(27.0_f64) * t24972 * t2319 + F::cast_from(0.135e2_f64) * t7423 * t2363 + t23886 + t23888 + t23890 + t23892 + t23895 + t23898 + t23900;
    (t24954, t24955, t24969, t24972, t24977)
}
