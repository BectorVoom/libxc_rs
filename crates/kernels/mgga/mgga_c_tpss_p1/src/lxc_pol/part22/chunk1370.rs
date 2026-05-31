//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1370/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1370<F: Float>(t1844: F, t30367: F, t5798: F, t645: F, t507: F, t6435: F, t12679: F, t13244: F, t1339: F, t13965: F, t1760: F, t1800: F, t18539: F, t18547: F, t18628: F, t18690: F, t18898: F, t19001: F, t19305: F, t19579: F, t19609: F, t19620: F, t20221: F, t20357: F, t24128: F, t25469: F, t3166: F, t3493: F, t3502: F, t44045: F, t44070: F, t5801: F, t5809: F, t61801: F, t6243: F, t626: F, t6323: F, t65085: F, t65094: F, t65097: F, t65899: F, t65924: F) -> (F, F) {
    let t67246 = t1844 * t30367;
    let t67250 = t5798 * t645;
    let t67270 = t507 * t6435;
    let t67274 = -F::cast_from(2.0_f64) * t626 * t3166 * t6323 - F::cast_from(2.0_f64) * t5801 * t13244 - F::cast_from(6.0_f64) * t18547 * t24128 * t19609 - F::cast_from(6.0_f64) * t61801 * t20221 + F::cast_from(6.0_f64) * t18547 * t20357 * t44045 - F::cast_from(6.0_f64) * t18547 * t18690 * t44070 - F::cast_from(6.0_f64) * t18547 * t25469 * t12679 - F::cast_from(6.0_f64) * t18547 * t24128 * t13965 - F::cast_from(6.0_f64) * t19579 * t67246 * t65899 - F::cast_from(4.0_f64) * t67250 * t1339 - F::cast_from(4.0_f64) * t18898 * t3502 - F::cast_from(2.0_f64) * t3493 * t18628 - F::cast_from(2.0_f64) * t65094 * t1800 - F::cast_from(4.0_f64) * t65097 * t1800 - F::cast_from(4.0_f64) * t19305 * t5809 + t6243 * t19001 - F::cast_from(6.0_f64) * t19620 * t18690 * t65924 + F::cast_from(12.0_f64) * t18547 * t20357 * t65085 + F::cast_from(6.0_f64) * t1760 * t67270 * t18539;
    (t67250, t67274)
}
