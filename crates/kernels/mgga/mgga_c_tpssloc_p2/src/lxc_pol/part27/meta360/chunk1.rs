//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1484/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1484<F: Float>(t1484: F, t828: F, t2647: F, t13350: F, t1516: F, t9993: F, t2696: F, t4166: F, t849: F, t13176: F, t842: F, t9601: F) -> (F, F, F, F, F, F, F) {
    let t13351 = t1484 * t828;
    let t13352 = t13351 * t2647;
    let t13353 = t13350 * t13352;
    let t13359 = F::new(7.0) / F::new(576.0) * t9993 * t1516;
    let t13360 = t4166 * t2696;
    let t13362 = F::new(7.0) / F::new(576.0) * t13360 * t849;
    let t13365 = t13176 * t842;
    let t13368 = t9601 * t1516;
    (t13351, t13352, t13353, t13359, t13362, t13365, t13368)
}
