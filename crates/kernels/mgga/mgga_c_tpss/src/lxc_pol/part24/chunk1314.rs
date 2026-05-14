//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1314/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1314<F: Float>(t226: F, t4783: F, t782: F, t1378: F, t3721: F, t4759: F, t818: F, t1379: F, t3664: F, t44960: F, t21291: F, t219: F, t17993: F, t18006: F, t18007: F, t19748: F, t19749: F, t19762: F, t19767: F, t19768: F, t19781: F, t21307: F, t21308: F, t21312: F, t21313: F, t21321: F, t21339: F, t4799: F, t52460: F, t5562: F, t5571: F, t5572: F, t5577: F, t61195: F, t61222: F, t61226: F, t61232: F, t6138: F, t64060: F, t64135: F, t64159: F, t69897: F, t819: F) -> (F,) {
    let t70103 = t4783 * t782 * t226;
    let t70113 = t3721 * t1378 * t226;
    let t70123 = t4759 * t818;
    let t70130 = t1379 * t3664;
    let t70134 = t44960 * t226;
    let t70144 = t21291 * t219;
    let t70160 = -4.0 * t64060 * t19749 + 6.0 * t61226 * t18007 * t70103 - 4.0 * t61222 * t21313 - 4.0 * t18006 * t61232 * t21312 - 4.0 * t18006 * t18007 * t70113 - 2.0 * t19767 * t19768 * t52460 + 2.0 * t19767 * t64159 * t19781 - 2.0 * t18006 * t18007 * t70123 - 4.0 * t18006 * t64159 * t19748 + 2.0 * t19767 * t18007 * t70130 + t19767 * t18007 * t70134 + t17993 * t21339 + t5571 * t5577 * t69897 * t226 - 4.0 * t18006 * t64159 * t19762 - t70144 * t819 + 4.0 * t64135 * t6138 + 2.0 * t17993 * t21321 - 6.0 * t17993 * t21308 + 2.0 * t5571 * t5572 * t5562 * t4799 + 24.0 * t5571 * t61195 * t21307 * t818;
    (t70160,)
}
