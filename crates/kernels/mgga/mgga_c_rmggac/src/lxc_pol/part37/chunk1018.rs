//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1018/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1018<F: Float>(t15496: F, t2160: F, t638: F, t739: F, t8264: F, t8975: F, t76041: F, t76043: F, t639: F, t640: F, t9595: F, t76046: F) -> (F, F, F, F, F, F) {
    let t78456 = t638 * t2160 * t15496;
    let t78457 = F::cast_from(0.40650199722100037752e-3_f64) * t78456;
    let t78462 = F::cast_from(0.11974241701863808564e0_f64) * t739 * t8264 * t8975;
    let t78464 = F::cast_from(0.2553875993597870364e-4_f64) * t76041;
    let t78465 = F::cast_from(0.14967802127329760705e-1_f64) * t76043;
    let t78468 = t638 * t639 * t640 * t9595;
    let t78469 = F::cast_from(0.15243824895787514157e-3_f64) * t78468;
    let t78473 = F::cast_from(0.20455996240684006298e-1_f64) * t76046;
    (t78457, t78462, t78464, t78465, t78469, t78473)
}
