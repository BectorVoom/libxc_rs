//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 987/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk987<F: Float>(t26291: F, t77327: F, t14444: F, t1635: F, t29838: F, t71219: F, t74969: F, t74975: F, t74979: F, t77299: F, t77300: F, t77301: F, t77303: F, t77305: F, t77309: F, t77313: F, t77317: F, t77321: F, t77322: F, t77323: F) -> (F, F) {
    let t77329 = F::new(0.35922725105591425692e0) * t26291 * t77327;
    let t77330 = t14444 * t1635;
    let t77332 = F::new(0.47896966807455234256e0) * t29838 * t77330;
    let t77333 = t77299 - t77300 + t77301 + t77303 - t77305 - t77309 + t77313 - t77317 + t77321 - t77322 - t71219 + t77323 - F::new(0.46594213659335792124e-1) * t74969 + F::new(0.93188427318671584248e-1) * t74975 + F::new(0.15531404553111930708e-1) * t74979 - t77329 + t77332;
    (t77330, t77333)
}
