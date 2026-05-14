//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1155/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1155<F: Float>(t10697: F, t2787: F, t10696: F, t2842: F, t2844: F, t912: F, t10702: F, t10704: F, t2793: F, t2836: F, t2775: F, t39103: F, t123: F, t882: F) -> (F, F, F, F, F) {
    let t41627 = 4.0 * t2787 * t10697;
    let t41635 = 0.64327917994770140268e2 * t2842 * t10696 * t2844 * t912;
    let t41639 = 0.3103560775156404018e4 * t10702 * t2793 * t10704 * t2836;
    let t41640 = t2775 * t39103;
    let t41642 = t123 * t882 * t41640;
    (t41627, t41635, t41639, t41640, t41642)
}
