//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 906/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk906<F: Float>(t3: F, t8240: F, t1458: F, t2180: F, t1401: F, t3941: F, t5371: F, t577: F, t8161: F, t8230: F, t590: F, t60: F) -> (F, F, F, F) {
    let t8241 = t3 * t8240;
    let t8251 = t2180 * t1458;
    let t8256 = F::new(0.45e1) * t8240 * t577 + F::new(0.135e2) * t8161 * t1458 + F::new(0.135e2) * t5371 * t2180 + F::new(27.0) * t3941 * t8251 + F::new(0.135e2) * t1401 * t8230;
    let t8705 = F::new(1.0) / t60 / t590;
    (t8241, t8251, t8256, t8705)
}
