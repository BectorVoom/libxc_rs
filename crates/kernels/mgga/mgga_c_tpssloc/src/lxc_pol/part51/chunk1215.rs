//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1215/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1215<F: Float>(t31688: F, t33115: F, t12571: F, t31687: F, t8515: F, t115889: F, t115907: F, t119938: F, t119944: F, t119952: F, t119965: F, t121074: F, t121081: F, t121087: F, t121094: F, t121099: F, t121102: F, t121105: F, t121108: F, t31019: F, t31672: F, t31675: F, t31677: F, t31681: F, t31693: F, t33560: F, t33572: F, t46104: F, t8511: F, t8512: F) -> (F,) {
    let t121121 = t31688 * t33115;
    let t121124 = t12571 * t31687 * t8515;
    let t121126 = 5.0 / 12.0 * t31675 * t121074 - 5.0 / 36.0 * t31672 * t33572 - 5.0 / 36.0 * t8512 * t121081 - 5.0 / 36.0 * t8512 * t119952 - 5.0 / 36.0 * t8512 * t121087 + 5.0 / 12.0 * t31675 * t119938 - 5.0 / 36.0 * t8512 * t119944 + 5.0 / 12.0 * t121094 * t31677 - 5.0 / 36.0 * t33560 * t31693 + 5.0 / 9.0 * t31681 * t121099 + 5.0 / 9.0 * t31681 * t121102 - 5.0 / 3.0 * t115907 * t121105 - 5.0 / 3.0 * t115907 * t121108 - 20.0 / 27.0 * t115889 - 5.0 / 72.0 * t31672 * t33115 - 5.0 / 72.0 * t8512 * t119965 - 5.0 / 72.0 * t46104 * t8511 * t8515 - 5.0 / 72.0 * t33560 * t31019 + 5.0 / 27.0 * t121121 + 5.0 / 27.0 * t121124;
    (t121126,)
}
