//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 575/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk575<F: Float>(t14259: F, t14303: F, t14306: F, t14312: F, t14431: F, t14432: F, t14433: F, t14440: F, t14443: F, t14447: F, t14450: F, t14454: F, t14457: F, t14460: F, t14461: F, t14462: F, t14463: F, t14464: F, t14468: F, t14471: F, t14500: F) -> (F, F) {
    let t14996 = F::cast_from(0.58171619854173713844e-5_f64) * t14259;
    let t15000 = t14431 - t14432 - t14433 - t14440 - t14443 + t14447 - t14450 - t14454 + t14457 + t14460 - t14461 + t14462 - t14463 - t14464 - F::cast_from(0.93188427318671584242e-2_f64) * t14303 + F::cast_from(0.15531404553111930707e-1_f64) * t14306 + F::cast_from(0.31062809106223861414e-2_f64) * t14312 + t14468 + t14471 - t14500;
    (t14996, t15000)
}
