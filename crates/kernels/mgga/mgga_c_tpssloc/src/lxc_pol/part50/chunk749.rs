//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 749/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk749<F: Float>(t1873: F, t3938: F, t671: F, t3941: F, t1401: F, t6534: F, t577: F, t7002: F, t7010: F, t33: F, t3953: F, t1437: F, t79: F) -> (F, F, F, F) {
    let t7014 = F::new(0.135e2) * t3938 * t1873;
    let t7015 = t1873 * t671;
    let t7017 = F::new(27.0) * t3941 * t7015;
    let t7019 = F::new(0.135e2) * t1401 * t6534;
    let t7020 = F::new(0.45e1) * t7002 * t577 + F::new(0.135e2) * t7010 * t671 + t7014 + t7017 + t7019;
    let t7428 = t3953 * t33;
    let t7431 = t79 * t1437;
    (t7015, t7020, t7428, t7431)
}
