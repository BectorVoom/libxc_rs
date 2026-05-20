//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2191/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2191<F: Float>(t28821: F, t7000: F, t1983: F, t24990: F, t26167: F, t7687: F, t91620: F, t28002: F, t6535: F, t12725: F, t7461: F, t19456: F) -> (F, F, F, F, F, F) {
    let t97836 = t28821 * t7000;
    let t97839 = F::new(6.0) * t1983 * t26167 * t24990;
    let t97842 = F::new(6.0) * t1983 * t91620 * t7687;
    let t97844 = F::new(4.0) * t28002 * t6535;
    let t97846 = F::new(4.0) * t12725 * t7461;
    let t97848 = F::new(4.0) * t19456 * t7461;
    (t97836, t97839, t97842, t97844, t97846, t97848)
}
