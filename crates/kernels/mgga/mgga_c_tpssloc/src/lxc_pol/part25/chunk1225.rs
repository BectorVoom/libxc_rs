//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1225/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1225<F: Float>(t2105: F, t3931: F, t1404: F, t7222: F, t24447: F, t580: F, t2098: F, t3946: F, t1395: F, t7240: F, t12513: F, t12537: F, t1396: F, t1398: F, t2099: F, t24448: F, t24486: F, t3: F, t3932: F, t7223: F, t84031: F, t85372: F, t85375: F) -> F {
    let t85379 = t3931 * t2105;
    let t85381 = t7222 * t1404;
    let t85392 = t24447 * t580;
    let t85394 = t2098 * t3946;
    let t85397 = t1395 * t7240;
    let tv4rho3sigma1 = t3 * t580 * t85372 + t12513 * t2105 + t12537 * t2099 + F::new(3.0) * t1396 * t24486 + t1398 * t85375 + F::new(3.0) * t1404 * t24448 + F::new(3.0) * t3932 * t7240 + F::new(3.0) * t3946 * t7223 + F::new(3.0) * t84031 + F::new(3.0) * t85379 + F::new(6.0) * t85381 + F::new(3.0) * t85392 + F::new(3.0) * t85394 + F::new(6.0) * t85397;
    tv4rho3sigma1
}
