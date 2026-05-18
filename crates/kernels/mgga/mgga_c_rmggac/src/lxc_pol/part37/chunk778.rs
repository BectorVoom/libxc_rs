//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 778/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk778<F: Float>(t14245: F, t73692: F, t21708: F, t21709: F, t9197: F, t15214: F, t68528: F, t14116: F, t14117: F, t8496: F, t21713: F, t21714: F, t9054: F) -> (F, F, F, F, F) {
    let t74060 = t73692 * t14245;
    let t74063 = t21708 * t21709 * t9197;
    let t74065 = t68528 * t15214;
    let t74069 = t14116 * t14117 * t8496;
    let t74072 = t21713 * t21714 * t9054;
    (t74060, t74063, t74065, t74069, t74072)
}
