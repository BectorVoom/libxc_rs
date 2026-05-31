//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2560/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2560<F: Float>(t4745: F, t64257: F, t4786: F, t63755: F, t14838: F, t18255: F, t14850: F, t18259: F, t11303: F, t1136: F, t11361: F, t11420: F, t15146: F, t15207: F, t1683: F, t1694: F, t18615: F, t18623: F, t18631: F, t18634: F, t18893: F, t21839: F, t21842: F, t21952: F, t3332: F, t3357: F, t3401: F, t4819: F, t4820: F, t4857: F, t51376: F, t6037: F, t6052: F, t63533: F) -> (F, F, F, F, F) {
    let t71784 = F::cast_from(6.0_f64) * t64257 * t4745;
    let t71786 = F::cast_from(0.48245938496077605201e2_f64) * t63755 * t4786;
    let t71788 = F::cast_from(6.0_f64) * t14838 * t18255;
    let t71790 = F::cast_from(0.48245938496077605201e2_f64) * t14850 * t18259;
    let t71791 = F::cast_from(0.30762056574649219972e4_f64) * t51376 * t18623 + F::cast_from(0.51947577317044391277e2_f64) * t11361 * t21839 + F::cast_from(0.51947577317044391277e2_f64) * t3401 * t63533 * t1694 + F::cast_from(0.51947577317044391277e2_f64) * t3401 * t18615 * t4857 + F::cast_from(18.0_f64) * t15146 * t18631 - F::cast_from(12.0_f64) * t15207 * t18634 - F::cast_from(24.0_f64) * t11420 * t21952 * t1136 + F::cast_from(18.0_f64) * t3357 * t6037 * t4819 - F::cast_from(6.0_f64) * t11303 * t21842 - F::cast_from(6.0_f64) * t3332 * t4820 * t6052 - F::cast_from(6.0_f64) * t3332 * t1683 * t18893 + t71784 - t71786 + t71788 - t71790;
    (t71784, t71786, t71788, t71790, t71791)
}
