//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1968/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1968<F: Float>(t26193: F, t6907: F, t1985: F, t225: F, t5318: F, t567: F, t214: F, t1377: F, t1842: F, t1307: F, t22635: F, t22633: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26206 = t26193 * t6907;
    let t26207 = t1985 * t26206;
    let t26210 = t5318 * t225 * t567;
    let t26211 = t214 * t26210;
    let t26212 = t1985 * t26211;
    let t26214 = t1377 * t1842;
    let t26215 = t26214 * t1307;
    let t26216 = t22635 * t26215;
    let t26217 = t22633 * t26216;
    (t26206, t26207, t26210, t26211, t26212, t26214, t26215, t26216, t26217)
}
