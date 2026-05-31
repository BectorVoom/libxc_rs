//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1201/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1201<F: Float>(t225: F, t24064: F, t81398: F, t12026: F, t1386: F, t2092: F, t24092: F, t24139: F, t26224: F, t26989: F, t3752: F, t3882: F, t39919: F, t568: F, t7191: F, t81379: F, t81386: F, t81393: F, t81395: F) -> F {
    let t84700 = t24064 * t225;
    let t84705 = F::cast_from(0.27415567780803773942e-2_f64) * t81398;
    let t84708 = -F::cast_from(18.0_f64) * t26224 * t26989 * t12026 - F::cast_from(0.49348022005446793095e-1_f64) * t81379 + F::cast_from(3.0_f64) * t3752 * t7191 * t568 + F::cast_from(0.9869604401089358619e-1_f64) * t81386 - F::cast_from(18.0_f64) * t3882 * t24092 - F::cast_from(0.23029076935875170111e0_f64) * t81393 - F::cast_from(3.0_f64) * t84700 * t1386 - t39919 * t2092 + F::cast_from(0.23029076935875170111e0_f64) * t81395 - t84705 - F::cast_from(3.0_f64) * t3882 * t24139;
    t84708
}
