//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 900/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk900<F: Float>(t78456: F, t739: F, t8264: F, t8975: F, t76041: F, t76043: F, t638: F, t639: F, t640: F, t9595: F, t15421: F, t4965: F, t70320: F, t71775: F, t76036: F, t78084: F, t78438: F, t78439: F, t78440: F, t78444: F, t78446: F, t78451: F, t78454: F) -> (F,) {
    let t78457 = 0.40650199722100037752e-3 * t78456;
    let t78462 = 0.11974241701863808564e0 * t739 * t8264 * t8975;
    let t78464 = 0.2553875993597870364e-4 * t76041;
    let t78465 = 0.14967802127329760705e-1 * t76043;
    let t78468 = t638 * t639 * t640 * t9595;
    let t78469 = 0.15243824895787514157e-3 * t78468;
    let t78470 = t78438 - t78439 + t78440 + 0.39914139006212695214e-1 * t4965 * t15421 + t78444 + t78446 - t78451 - 0.17519306092901367187e-5 * t76036 + t78454 - t78457 - t71775 - 0.59871208509319042821e-1 * t739 * t78084 + t78462 - 0.49700494569958178265e-1 * t70320 - t78464 - t78465 + t78469;
    (t78470,)
}
