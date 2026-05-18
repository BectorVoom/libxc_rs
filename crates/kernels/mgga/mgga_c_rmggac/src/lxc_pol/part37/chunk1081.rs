//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1081/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1081<F: Float>(t75572: F, t75590: F, t75593: F, t69871: F, t73382: F, t73383: F, t75596: F, t75602: F, t77664: F, t77665: F, t77666: F, t77669: F, t77670: F, t77672: F, t77677: F, t77679: F, t77681: F) -> F {
    let t80264 = F::new(0.15372131649401827112e-4) * t75572;
    let t80265 = F::new(0.17347588262831798124e-4) * t75590;
    let t80266 = F::new(0.17347588262831798124e-4) * t75593;
    let t80268 = -t73382 - t73383 - t80264 - t77664 - t77665 + t77666 + t77669 - t77670 - t77672 + t80265 + t80266 + t69871 - F::new(0.81756761766873046873e-6) * t75596 + t77677 + t75602 - t77679 - t77681;
    t80268
}
