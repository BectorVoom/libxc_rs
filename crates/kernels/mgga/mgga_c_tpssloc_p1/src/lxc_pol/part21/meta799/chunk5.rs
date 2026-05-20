//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2784/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2784<F: Float>(t39463: F, t39468: F, t39472: F, t39476: F, t40721: F, t40732: F, t57959: F, t57961: F, t57962: F, t57966: F, t57970: F, t57972: F, t57975: F, t57983: F, t57986: F, t57987: F, t57988: F, t57989: F, t57990: F) -> F {
    let t58966 = t57959 + t57961 - t57962 + t57966 + t57970 + t39463 - t39468 + t57972 + t57975 - t40721 - t57983 - t39472 - t39476 + t57986 - t57987 + t57988 + t57989 - t40732 - t57990;
    t58966
}
