//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2636/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2636<F: Float>(t22398: F, t225: F, t1243: F, t72361: F, t1235: F, t22298: F, t11907: F, t11914: F, t11915: F, t1215: F, t15027: F, t15245: F, t19128: F, t19129: F, t19131: F, t19142: F, t19157: F, t19160: F, t22341: F, t22348: F, t22354: F, t22372: F, t22390: F, t3604: F, t3624: F, t44724: F, t44726: F, t5064: F, t53565: F) -> (F, F, F, F) {
    let t73613 = t22398 * t225;
    let t73630 = t72361 * t1243;
    let t73663 = t1235 * t22298;
    let t73670 = F::new(24.0) * t1215 * t22348 * t44724 * t44726 + t11914 * t11915 * t73663 - F::new(3.0) * t19128 * t22354 * t3624 - F::new(3.0) * t11907 * t22372 + F::new(12.0) * t15027 * t19142 - F::new(6.0) * t15245 * t19131 - F::new(3.0) * t15245 * t19160 + F::new(3.0) * t19129 * t5064 - F::new(18.0) * t19157 * t53565 + F::new(3.0) * t22341 * t3604 + F::new(3.0) * t22390 * t3604;
    (t73613, t73630, t73663, t73670)
}
