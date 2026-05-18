//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 935/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk935<F: Float>(t31: F, t625: F, t240: F, t8307: F, t8513: F, t8663: F, t111: F, t8843: F, t7537: F, t857: F, t32815: F, t81591: F) -> (F, F, F, F, F) {
    let t117496 = t625 * t31;
    let t117527 = F::new(55.0) / F::new(81.0) * t8663 * t8513 * t8307 * t240;
    let t117687 = t8843 * t111;
    let t118472 = t857 * t7537;
    let t118480 = t81591 * t32815;
    (t117496, t117527, t117687, t118472, t118480)
}
