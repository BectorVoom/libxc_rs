//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 928/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk928<F: Float>(t76786: F, t2211: F, t40983: F, t739: F, t15672: F, t4041: F, t14391: F, t17859: F, t14396: F, t41059: F, t73949: F, t73953: F) -> (F, F, F, F, F, F, F, F) {
    let t76787 = F::new(0.42564599893297839398e-5) * t76786;
    let t76790 = F::new(0.11974241701863808564e0) * t739 * t2211 * t40983;
    let t76792 = F::new(0.11974241701863808564e0) * t4041 * t15672;
    let t76793 = t17859 * t14391;
    let t76794 = F::new(0.12769379967989351819e-4) * t76793;
    let t76795 = t17859 * t14396;
    let t76796 = F::new(0.85129199786595678796e-5) * t76795;
    let t76799 = F::new(0.11974241701863808564e0) * t739 * t2211 * t41059;
    let t76800 = F::new(0.16263363996404810741e-4) * t73949;
    let t76801 = F::new(0.43368970657079495308e-4) * t73953;
    (t76787, t76790, t76792, t76794, t76796, t76799, t76800, t76801)
}
