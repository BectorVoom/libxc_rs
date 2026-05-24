//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 655/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk655<F: Float>(t2466: F, t4041: F, t2435: F, t4965: F, t2262: F, t623: F, t570: F, t8264: F, t1356: F, t1668: F, t2265: F, t2228: F, t551: F) -> (F, F, F, F, F, F, F) {
    let t9410 = t4041 * t2466;
    let t9423 = t4965 * t2435;
    let t9425 = t623 * t2262;
    let t9427 = t8264 * t570;
    let t9428 = t1356 * t9427;
    let t9435 = t1668 * t2265;
    let t9437 = t2228 * t551;
    (t9410, t9423, t9425, t9427, t9428, t9435, t9437)
}
