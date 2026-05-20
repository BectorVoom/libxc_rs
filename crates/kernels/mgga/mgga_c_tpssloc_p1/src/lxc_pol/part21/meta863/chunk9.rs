//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3150/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3150<F: Float>(t3030: F, t6150: F, t3609: F, t3623: F, t5011: F, t491: F, t63280: F, t64446: F, t64454: F, t64456: F, t64458: F, t64460: F, t64462: F, t64464: F, t64466: F, t64470: F, t64472: F, t64475: F) -> (F, F, F, F, F, F) {
    let t65253 = t6150 * t3030;
    let t65254 = t65253 * t3609;
    let t65262 = t65253 * t3623;
    let t65264 = t5011 * t5011;
    let t65265 = t491 * t65264;
    let t65278 = t64446 - t64454 - t64456 - t64458 - t64460 - t64462 - t64464 + t64466 + t64470 + t63280 + t64472 + t64475;
    (t65253, t65254, t65262, t65264, t65265, t65278)
}
