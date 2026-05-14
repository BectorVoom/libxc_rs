//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1301/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1301<F: Float>(t43819: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t43811: F, t43816: F, t43823: F, t43828: F, t1099: F, t1118: F, t44021: F, t44036: F, t44052: F) -> (F,) {
    let t44053 = 0.31003950617283950618e1 * t43819;
    let t44067 = t44053 + 0.79724444444444444446e0 * t43780 + 0.15944888888888888889e1 * t43782 + 0.15944888888888888889e1 * t43784 - 0.23917333333333333333e1 * t43786 - 0.39862222222222222223e0 * t43788 + 0.39862222222222222223e1 * t43794 - 0.71752000000000000002e1 * t43798 + 0.71752e1 * t43802 + 0.29896666666666666667e0 * t43806 - 0.88582716049382716048e0 * t43811 - 0.12401580246913580247e1 * t43816 - 0.59793333333333333333e0 * t43823 + 0.17938e1 * t43828;
    let t44072 = 1.0 * t1099 * (t44021 + t44036 + t44052 + t44067) * t1118;
    (t44072,)
}
