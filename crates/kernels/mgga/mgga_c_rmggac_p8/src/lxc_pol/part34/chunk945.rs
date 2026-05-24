//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 945/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk945<F: Float>(t73963: F, t73966: F, t70797: F, t73936: F, t76779: F, t76780: F, t76781: F, t76787: F, t76790: F, t76792: F, t76794: F, t76796: F, t76799: F, t76800: F, t76801: F, t76802: F, t76803: F) -> F {
    let t76804 = F::cast_from(0.81823984962736025192e-1_f64) * t73963;
    let t76805 = F::cast_from(0.40911992481368012596e-1_f64) * t73966;
    let t76806 = F::cast_from(0.87596530464506835935e-6_f64) * t73936 + t76779 - t76780 - t76781 + t76787 + t76790 - t76792 + t70797 + t76794 + t76796 + t76799 + t76800 + t76801 - t76802 - t76803 + t76804 + t76805;
    t76806
}
