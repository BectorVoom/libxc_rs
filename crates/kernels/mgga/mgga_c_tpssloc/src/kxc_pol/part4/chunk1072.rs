//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1072/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1072<F: Float>(t11979: F, t15890: F, t15895: F, t588: F, t6328: F, t592: F, t11984: F, t15880: F, t15889: F, t15894: F, t19543: F, t19574: F, t19576: F, t19577: F, t19581: F, t3918: F, t3919: F, t5122: F, t5126: F, t5161: F, t5187: F, t5308: F, t6347: F, t9457: F, t9476: F, t9484: F) -> (F, F, F, F, F, F) {
    let t19588 = 32.0 * t11979;
    let t19589 = 0.34631718211362927517e2 * t15890;
    let t19590 = 0.11696447245269292414e1 * t15895;
    let t19591 = t588 * t6328;
    let t19592 = 4.0 * t19591;
    let t19593 = t592 * t6328;
    let t19594 = 4.0 * t19593;
    let t19595 = -6.0 * t19577 * t3918 * t5161 + 3.0 * t3918 * t3919 * t6347 + 6.0 * t3918 * t5122 * t5187 + 12.0 * t5122 * t5126 * t5308 - t11984 - t15880 + t15889 - t15894 - t19543 + t19574 + t19576 - t19581 + t19588 - t19589 - t19590 + t19592 - t19594 - t9457 + t9476 + t9484;
    (t19588, t19589, t19590, t19592, t19594, t19595)
}
