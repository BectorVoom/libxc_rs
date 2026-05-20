//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2218/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2218<F: Float>(t14507: F, t23540: F, t23433: F, t4630: F, t10189: F, t1920: F, t4343: F, t13783: F, t4338: F, t13546: F, t13555: F, t13559: F, t14099: F, t14103: F, t14167: F, t1618: F, t23541: F, t25571: F, t25574: F, t2987: F, t3043: F, t4509: F, t6680: F, t6765: F, t82964: F) -> F {
    let t88600 = t14507 * t23540;
    let t88604 = t23433 * t4630 / F::new(1152.0);
    let t88622 = t1920 * t10189 * t4343 / F::new(216.0);
    let t88625 = t1920 * t13783 * t4338 / F::new(324.0);
    let t88632 = -t88600 * t3043 / F::new(1536.0) + t88604 + F::new(19.0) / F::new(864.0) * t82964 * t1618 + t1920 * t2987 * t13559 / F::new(48.0) - t23541 * t14099 / F::new(768.0) - t23541 * t14103 / F::new(1536.0) + t6765 * t14167 / F::new(384.0) + t6680 * t25571 / F::new(27.0) - F::new(2.0) / F::new(81.0) * t6680 * t25574 - t88622 + t88625 - t1920 * t2987 * t13546 / F::new(144.0) - t1920 * t4509 * t13555 / F::new(36.0);
    t88632
}
