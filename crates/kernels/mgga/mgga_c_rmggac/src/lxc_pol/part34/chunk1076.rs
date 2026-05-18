//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1076/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1076<F: Float>(t78468: F, t15421: F, t4965: F, t70320: F, t71775: F, t739: F, t76036: F, t78084: F, t78438: F, t78439: F, t78440: F, t78444: F, t78446: F, t78451: F, t78454: F, t78457: F, t78462: F, t78464: F, t78465: F) -> F {
    let t78469 = F::new(0.15243824895787514157e-3) * t78468;
    let t78470 = t78438 - t78439 + t78440 + F::new(0.39914139006212695214e-1) * t4965 * t15421 + t78444 + t78446 - t78451 - F::new(0.17519306092901367187e-5) * t76036 + t78454 - t78457 - t71775 - F::new(0.59871208509319042821e-1) * t739 * t78084 + t78462 - F::new(0.49700494569958178265e-1) * t70320 - t78464 - t78465 + t78469;
    t78470
}
