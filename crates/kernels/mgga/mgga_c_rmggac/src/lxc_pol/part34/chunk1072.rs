//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1072/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1072<F: Float>(t70208: F, t14434: F, t70194: F, t70198: F, t71727: F, t739: F, t75968: F, t78394: F, t78395: F, t78397: F, t78399: F, t78400: F, t78401: F, t78402: F, t78403: F, t78404: F, t78405: F, t78406: F, t8377: F) -> F {
    let t78409 = F::new(0.79808624799933448875e-4) * t70208;
    let t78413 = F::new(0.72714524817717142308e-5) * t75968 - t78394 + t78395 + t78397 + t78399 - t78400 - t78401 - t78402 - t78403 + t78404 - t78405 + t78406 - t71727 + F::new(0.16566831523319392755e-1) * t70194 + F::new(0.82834157616596963775e-1) * t70198 - t78409 + F::new(0.11974241701863808564e0) * t739 * t14434 * t8377;
    t78413
}
