//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1294/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1294<F: Float>(t44053: F, t63361: F, t78057: F, t78084: F, t78087: F, t78090: F, t78093: F, t78095: F, t78097: F, t78100: F, t78103: F, t78105: F, t78107: F, t78109: F, t1099: F, t1118: F, t78147: F, t78162: F, t78177: F) -> (F,) {
    let t78191 = -0.10954222222222222222e0 * t78084 - 0.98587999999999999999e0 * t78087 + 0.65725333333333333332e0 * t78090 + 0.295764e1 * t78093 + 0.1898925e1 * t78095 + t44053 + 0.46074375e0 * t78097 + 0.21908444444444444444e0 * t78100 + 0.15944888888888888889e1 * t63361 + 0.614325e0 * t78103 - 0.379785e1 * t78105 + 0.85451625e1 * t78107 - 0.46074375e0 * t78109 - 0.71752000000000000002e1 * t78057;
    let t78196 = 1.0 * t1099 * (t78147 + t78162 + t78177 + t78191) * t1118;
    (t78196,)
}
