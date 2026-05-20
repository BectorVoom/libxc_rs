//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2364/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2364<F: Float>(t100871: F, t100873: F, t100875: F, t100879: F, t100883: F, t100885: F, t100887: F, t100890: F, t100893: F, t100897: F, t100899: F, t100902: F, t105102: F, t105105: F, t105108: F, t19534: F, t24969: F, t5456: F, t5493: F, t577: F, t671: F, t7423: F) -> F {
    let t105115 = F::new(0.45e1) * t105102 * t577 + F::new(0.135e2) * t105105 * t671 + t100871 + t100873 + t100875 + F::new(27.0) * t105108 * t5456 + F::new(0.135e2) * t7423 * t19534 + t100879 + F::new(0.135e2) * t24969 * t5493 + t100883 + t100885 + t100887 + t100890 + t100893 + t100897 + t100899 + t100902;
    t105115
}
