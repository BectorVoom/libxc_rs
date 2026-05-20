//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2375/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2375<F: Float>(t10662: F, t14395: F, t42028: F, t10829: F, t14258: F, t959: F, t10605: F, t4483: F, t10523: F, t2933: F, t4471: F, t47793: F, t47795: F, t47798: F, t47802: F, t48679: F, t48681: F, t48725: F, t48727: F, t48730: F, t48732: F, t48734: F, t48736: F, t48738: F, t48741: F, t48744: F, t48747: F) -> (F, F, F, F, F) {
    let t48750 = F::cast_from(0.62071215503128080361e4_f64) * t42028 * t14395 * t10662;
    let t48753 = F::cast_from(0.6233709278045326953e3_f64) * t959 * t14258 * t10829;
    let t48755 = F::cast_from(0.5848223622634646207e0_f64) * t4483 * t10605;
    let t48759 = F::cast_from(0.31168546390226634765e3_f64) * t959 * t10523 * t4471 * t2933;
    let t48760 = t47793 - t47795 + t47798 + t47802 - t48679 - t48681 - t48725 - t48727 - t48730 - t48732 - t48734 + t48736 + t48738 - t48741 - t48744 - t48747 - t48750 - t48753 - t48755 + t48759;
    (t48750, t48753, t48755, t48759, t48760)
}
