//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 882/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk882<F: Float>(t70819: F, t74060: F, t74069: F, t74072: F, t74075: F, t74078: F, t76842: F, t76843: F, t76844: F, t76846: F, t76848: F, t76849: F, t76854: F, t76855: F, t76856: F, t76857: F, t76858: F) -> (F,) {
    let t80034 = t76842 + t76843 - t76844 - t76846 + t70819 + 0.17451485956252114153e-4 * t74060 + t76848 - t76849 + 0.17519306092901367186e-5 * t74069 + 0.52557918278704101561e-6 * t74072 - 0.52557918278704101561e-6 * t74075 - 0.35038612185802734374e-6 * t74078 - t76854 + t76855 + t76856 - t76857 - t76858;
    (t80034,)
}
