//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 521/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk521<F: Float>(t14671: F, t3352: F, t3351: F, t14211: F, t14214: F, t14217: F, t14220: F, t209: F, t476: F, t698: F, t515: F, t1971: F, t1970: F, t14469: F, t739: F, t14227: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14672 = t3352 * t14671;
    let t14673 = t3351 * t14672;
    let t14674 = 0.12769379967989351819e-4 * t14673;
    let t14676 = 0.16263363996404810741e-4 * t14211;
    let t14677 = 0.16263363996404810741e-4 * t14214;
    let t14678 = 0.3252672799280962148e-5 * t14217;
    let t14679 = 0.3252672799280962148e-5 * t14220;
    let t14681 = t698 * t476 * t209;
    let t14682 = t515 * t14681;
    let t14683 = t1971 * t14682;
    let t14684 = t1970 * t14683;
    let t14685 = 0.42564599893297839398e-5 * t14684;
    let t14686 = t739 * t14469;
    let t14689 = 0.1276937996798935182e-4 * t14227;
    (t14672, t14674, t14676, t14677, t14678, t14679, t14683, t14685, t14686, t14689)
}
