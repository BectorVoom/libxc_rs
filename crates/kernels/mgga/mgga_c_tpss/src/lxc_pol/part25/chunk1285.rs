//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1285/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1285<F: Float>(t61087: F, t18005: F, t6134: F, t19733: F, t5570: F, t30: F, t31814: F, t2: F, t2436: F, t33: F, t1497: F, t1317: F, t5506: F) -> (F, F, F, F, F, F, F, F) {
    let t63998 = F::new(119.0) / F::new(864.0) * t61087;
    let t64060 = t6134 * t18005;
    let t64135 = t19733 * t5570;
    let t64247 = t31814 * t30;
    let t64300 = t2436 * t2;
    let t64879 = t31814 * t33;
    let t64975 = t2436 * t1497;
    let t65157 = t5506 * t1317;
    (t63998, t64060, t64135, t64247, t64300, t64879, t64975, t65157)
}
