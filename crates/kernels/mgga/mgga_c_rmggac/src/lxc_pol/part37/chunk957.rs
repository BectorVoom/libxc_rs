//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 957/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk957<F: Float>(t77316: F, t2046: F, t2050: F, t2475: F, t31: F, t71214: F, t71222: F, t14444: F, t1632: F, t26291: F, t1635: F, t29838: F) -> (F, F, F, F, F, F, F, F) {
    let t77317 = F::new(0.15243824895787514157e-3) * t77316;
    let t77320 = t2046 * t2050 * t2475 * t31;
    let t77321 = F::new(0.21684485328539747656e-4) * t77320;
    let t77322 = F::new(0.15243824895787514157e-3) * t71214;
    let t77323 = F::new(0.21684485328539747656e-4) * t71222;
    let t77327 = t14444 * t1632;
    let t77329 = F::new(0.35922725105591425692e0) * t26291 * t77327;
    let t77330 = t14444 * t1635;
    let t77332 = F::new(0.47896966807455234256e0) * t29838 * t77330;
    (t77317, t77321, t77322, t77323, t77327, t77329, t77330, t77332)
}
