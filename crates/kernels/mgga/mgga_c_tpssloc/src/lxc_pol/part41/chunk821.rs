//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 821/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk821<F: Float>(t2199: F, t671: F, t1401: F, t3938: F, t3941: F, t577: F, t8189: F, t8199: F, t8207: F, t1774: F) -> (F, F, F) {
    let t8212 = t2199 * t671;
    let t8217 = 0.45e1 * t8199 * t577 + 0.135e2 * t8207 * t671 + 0.135e2 * t3938 * t2199 + 27.0 * t3941 * t8212 + 0.135e2 * t1401 * t8189;
    let t8260 = t1774 * t2199;
    (t8212, t8217, t8260)
}
