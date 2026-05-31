//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1508/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1508<F: Float>(t5168: F, t592: F, t5166: F, t588: F, t5187: F, t571: F, t11981: F, t2528: F, t5154: F, t172: F, t5151: F, t763: F) -> (F, F, F, F, F, F, F) {
    let t15877 = t592 * t5168;
    let t15880 = F::cast_from(8.0_f64) * t588 * t5166;
    let t15883 = t571 * t5187;
    let t15889 = F::cast_from(32.0_f64) * t11981;
    let t15890 = t5154 * t2528;
    let t15892 = t5151 * t172;
    let t15894 = F::cast_from(0.11696447245269292414e1_f64) * t15892 * t763;
    (t15877, t15880, t15883, t15889, t15890, t15892, t15894)
}
