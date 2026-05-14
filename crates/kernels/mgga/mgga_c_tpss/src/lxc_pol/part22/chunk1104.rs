//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1104/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1104<F: Float>(t18670: F, t5489: F, t1791: F, t18351: F, t5492: F, t5791: F, t1675: F, t1792: F, t18305: F, t18338: F, t18347: F, t18350: F, t18356: F, t18360: F, t18363: F, t18366: F, t18648: F, t18649: F, t18652: F, t18661: F, t18663: F, t18666: F, t5483: F, t5785: F, t5794: F) -> (F, F, F, F) {
    let t18671 = t18670 * t5489;
    let t18673 = t1791 * t18351;
    let t18676 = t5492 * t5791;
    let t18678 = -10.0 / 3.0 * t5785 * t18356 - 5.0 / 3.0 * t5785 * t18360 - 2.0 / 3.0 * t18363 * t1792 - 2.0 / 3.0 * t18366 * t1792 - 4.0 / 3.0 * t5492 * t5794 + t18648 - 10.0 / 3.0 * t18649 * t5489 - 16.0 / 9.0 * t18652 - 4.0 / 3.0 * t18338 * t1792 + t18305 * t1792 / 3.0 + 2.0 / 3.0 * t5483 * t5794 - 16.0 / 9.0 * t18661 + t1675 * t18663 / 3.0 + 10.0 * t18666 * t18347 + 80.0 / 9.0 * t18671 + 20.0 / 3.0 * t18350 * t18673 + 32.0 / 9.0 * t18676;
    (t18671, t18673, t18676, t18678)
}
