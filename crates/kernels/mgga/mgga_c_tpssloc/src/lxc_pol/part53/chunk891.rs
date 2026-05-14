//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 891/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk891<F: Float>(t1985: F, t26202: F, t31611: F, t8606: F, t8944: F, t24994: F, t34076: F, t580: F, t111: F, t33915: F, t116437: F, t1983: F, t2095: F, t23938: F, t24987: F, t26880: F, t26898: F, t27143: F, t32187: F, t32189: F, t32203: F, t32235: F, t33234: F, t33363: F, t33855: F, t33900: F, t34067: F, t3701: F, t4026: F, t4073: F, t55242: F, t650: F, t672: F, t6876: F, t7057: F, t7218: F, t7685: F, t7687: F, t7796: F, t8607: F, t8774: F, t8805: F, t8808: F) -> (F, F, F, F, F, F) {
    let t122562 = t1985 * t31611 * t26202;
    let t122654 = t8606 * t8944;
    let t122698 = t8606 * t24994;
    let t123337 = t34076 * t580;
    let t123368 = t33915 * t111;
    let t123373 = 2.0 * t33363 * t7218 + t24987 * t8805 - t650 * t34067 + t7685 * t32187 - t4026 * t8774 + t6876 * t33855 + 6.0 * t8607 * t26898 - 2.0 * t8607 * t26880 + 2.0 * t7685 * t32203 - 4.0 * t33234 * t7057 - 4.0 * t23938 * t7796 + 3.0 * t1983 * t116437 * t7687 + 2.0 * t1983 * t8808 * t55242 - 2.0 * t1983 * t2095 * t3701 * t27143 - t7685 * t32189 - 2.0 * t6876 * t33900 - 2.0 * t123368 * t672 - 2.0 * t32235 * t4073;
    (t122562, t122654, t122698, t123337, t123368, t123373)
}
