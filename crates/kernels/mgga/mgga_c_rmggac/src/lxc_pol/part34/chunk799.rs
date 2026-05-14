//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 799/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk799<F: Float>(t76793: F, t14396: F, t17859: F, t2211: F, t41059: F, t739: F, t73949: F, t73953: F, t73957: F, t73960: F, t73963: F, t73966: F, t70797: F, t73936: F, t76779: F, t76780: F, t76781: F, t76787: F, t76790: F, t76792: F) -> (F,) {
    let t76794 = 0.12769379967989351819e-4 * t76793;
    let t76795 = t17859 * t14396;
    let t76796 = 0.85129199786595678796e-5 * t76795;
    let t76799 = 0.11974241701863808564e0 * t739 * t2211 * t41059;
    let t76800 = 0.16263363996404810741e-4 * t73949;
    let t76801 = 0.43368970657079495308e-4 * t73953;
    let t76802 = 0.30487649791575028312e-3 * t73957;
    let t76803 = 0.40911992481368012596e-1 * t73960;
    let t76804 = 0.81823984962736025192e-1 * t73963;
    let t76805 = 0.40911992481368012596e-1 * t73966;
    let t76806 = 0.87596530464506835935e-6 * t73936 + t76779 - t76780 - t76781 + t76787 + t76790 - t76792 + t70797 + t76794 + t76796 + t76799 + t76800 + t76801 - t76802 - t76803 + t76804 + t76805;
    (t76806,)
}
