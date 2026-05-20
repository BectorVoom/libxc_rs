//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1338/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1338<F: Float>(t13425: F, t13459: F, t858: F, t225: F, t4149: F, t13050: F, t13053: F, t13059: F, t13062: F, t13065: F, t13068: F, t13072: F, t13378: F, t259: F, t2597: F, t2713: F, t2720: F, t4268: F, t4273: F, t4301: F, t855: F, t866: F) -> (F, F, F, F) {
    let t13460 = t13425 + t13459;
    let t13461 = t858 * t13460;
    let t13463 = t4149 * t225;
    let t13470 = -F::new(6.0) * t13050 * t855 - F::new(2.0) * t13053 * t866 + F::new(2.0) * t13059 * t855 + F::new(2.0) * t13062 * t259 - F::new(2.0) * t13065 * t866 + F::new(2.0) * t13068 * t259 + F::new(4.0) * t13072 * t855 + t13378 * t259 - t13461 * t855 - F::new(2.0) * t13463 * t866 + F::new(4.0) * t2597 * t4273 - F::new(2.0) * t2713 * t4301 + F::new(2.0) * t2720 * t4268;
    (t13460, t13461, t13463, t13470)
}
