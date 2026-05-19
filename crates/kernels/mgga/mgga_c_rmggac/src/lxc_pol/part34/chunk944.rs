//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 944/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk944<F: Float>(t15672: F, t4041: F, t14391: F, t17859: F, t14396: F, t2211: F, t41059: F, t739: F, t73949: F, t73953: F, t73957: F, t73960: F) -> (F, F, F, F, F, F, F, F) {
    let t76792 = F::cast_from(0.11974241701863808564e0_f64) * t4041 * t15672;
    let t76793 = t17859 * t14391;
    let t76794 = F::cast_from(0.12769379967989351819e-4_f64) * t76793;
    let t76795 = t17859 * t14396;
    let t76796 = F::cast_from(0.85129199786595678796e-5_f64) * t76795;
    let t76799 = F::cast_from(0.11974241701863808564e0_f64) * t739 * t2211 * t41059;
    let t76800 = F::cast_from(0.16263363996404810741e-4_f64) * t73949;
    let t76801 = F::cast_from(0.43368970657079495308e-4_f64) * t73953;
    let t76802 = F::cast_from(0.30487649791575028312e-3_f64) * t73957;
    let t76803 = F::cast_from(0.40911992481368012596e-1_f64) * t73960;
    (t76792, t76794, t76796, t76799, t76800, t76801, t76802, t76803)
}
