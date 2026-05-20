//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3209/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3209<F: Float>(t19253: F, t225: F, t5088: F, t11925: F, t1238: F, t1241: F, t1251: F, t1252: F, t14980: F, t15786: F, t15803: F, t15820: F, t1760: F, t1761: F, t19208: F, t19220: F, t19234: F, t3593: F, t3598: F, t3599: F, t3631: F, t45350: F, t466: F, t498: F, t5055: F, t5060: F, t51925: F, t51928: F, t6243: F, t6268: F, t65208: F, t65249: F, t65343: F, t65374: F, t65408: F, t66675: F, t66702: F, t66737: F, t66769: F, t66802: F) -> F {
    let t66822 = t19253 * t225;
    let t66825 = t5088 * t5088;
    let t66842 = -F::new(2.0) * t65208 * t1252 + F::new(4.0) * t5055 * t15803 - t1238 * t1241 * (t65249 + t65343 + t65374 + t65408 + t66702 + t66737 + t66769 + t66802) + F::new(24.0) * t1238 * t45350 * t6243 * t3599 + F::new(8.0) * t15820 * t5060 - F::new(2.0) * t19234 * t3631 + F::new(4.0) * t1238 * t3598 * t19208 * t1251 + F::new(8.0) * t14980 * t5060 - F::new(4.0) * t66822 * t1252 + F::new(4.0) * t1238 * t3598 * t66825 + t466 * t66675 * t498 - F::new(2.0) * t51928 * t1761 - t11925 * t6268 + F::new(4.0) * t3593 * t19220 + F::new(4.0) * t1238 * t3598 * t1760 * t15786 - F::new(4.0) * t51925 * t1761;
    t66842
}
