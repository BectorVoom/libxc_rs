//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1397/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1397<F: Float>(t3109: F, t4630: F, t3108: F, t4640: F, t1611: F, t3047: F, t3103: F, t4641: F, t1040: F, t4616: F, t1612: F, t3082: F) -> (F, F, F, F, F, F) {
    let t14059 = t3109 * t4630 / F::new(432.0);
    let t14077 = t4640 * t3108;
    let t14080 = t1611 * t3047;
    let t14084 = t4641 * t3103 / F::new(2304.0);
    let t14085 = t4616 * t1040;
    let t14117 = t1612 * t3082;
    (t14059, t14077, t14080, t14084, t14085, t14117)
}
