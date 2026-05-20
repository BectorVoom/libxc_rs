//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1825/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1825<F: Float>(t23384: F, t6707: F, t6695: F, t6680: F, t6683: F, t6699: F, t968: F, t1920: F, t225: F, t3173: F) -> (F, F, F, F, F, F) {
    let t23385 = t23384 * t6707;
    let t23387 = t23384 * t6695;
    let t23389 = t6680 * t6683;
    let t23391 = t968 * t6699;
    let t23392 = t1920 * t23391;
    let t23394 = t225 * t3173;
    (t23385, t23387, t23389, t23391, t23392, t23394)
}
