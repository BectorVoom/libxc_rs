//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1189/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1189<F: Float>(t19591: F, t592: F, t6328: F, t11984: F, t15880: F, t15889: F, t15894: F, t19543: F, t19574: F, t19576: F, t19577: F, t19581: F, t19588: F, t19589: F, t19590: F, t3918: F, t3919: F, t5122: F, t5126: F, t5161: F, t5187: F, t5308: F, t6347: F, t9457: F, t9476: F, t9484: F) -> (F, F, F) {
    let t19592 = F::new(4.0) * t19591;
    let t19593 = t592 * t6328;
    let t19594 = F::new(4.0) * t19593;
    let t19595 = -F::new(6.0) * t19577 * t3918 * t5161 + F::new(3.0) * t3918 * t3919 * t6347 + F::new(6.0) * t3918 * t5122 * t5187 + F::new(12.0) * t5122 * t5126 * t5308 - t11984 - t15880 + t15889 - t15894 - t19543 + t19574 + t19576 - t19581 + t19588 - t19589 - t19590 + t19592 - t19594 - t9457 + t9476 + t9484;
    (t19592, t19594, t19595)
}
