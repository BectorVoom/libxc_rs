//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 961/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk961<F: Float>(t115539: F, t115550: F, t115566: F, t115629: F, t2105: F, t7222: F, t2098: F, t7240: F, t1395: F, t8822: F, t32311: F, t576: F) -> (F, F, F, F, F, F, F, F) {
    let t117284 = F::new(0.10417915756705434098e0) * t115539;
    let t117287 = F::new(0.3289868133696452873e-1) * t115550;
    let t117300 = F::new(0.25587863262083522346e0) * t115566;
    let t117317 = F::new(0.10417915756705434098e0) * t115629;
    let t117347 = t7222 * t2105;
    let t117349 = t2098 * t7240;
    let t117357 = t1395 * t8822;
    let t117359 = t576 * t32311;
    (t117284, t117287, t117300, t117317, t117347, t117349, t117357, t117359)
}
