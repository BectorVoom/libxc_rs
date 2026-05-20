//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1308/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1308<F: Float>(t13003: F, t13028: F, t252: F, t1492: F, t2710: F, t1519: F, t2591: F, t225: F, t4266: F, t10049: F, t1528: F, t259: F, t2597: F, t2713: F, t2720: F, t2743: F, t4147: F, t4268: F, t4273: F, t4301: F, t866: F, t9590: F, t9593: F) -> (F, F, F, F, F, F) {
    let t13029 = t13003 + t13028;
    let t13030 = t13029 * t252;
    let t13034 = t1492 * t2710;
    let t13036 = t2591 * t1519;
    let t13042 = t4266 * t225;
    let t13048 = -t10049 * t1528 + t13030 * t259 + t13034 * t259 + t13036 * t259 - F::new(2.0) * t13042 * t866 - t1528 * t9590 - F::new(2.0) * t1528 * t9593 - F::new(2.0) * t2597 * t4301 + F::new(4.0) * t2713 * t4273 + F::new(2.0) * t2720 * t4147 - t2743 * t4147 - t2743 * t4268;
    (t13029, t13030, t13034, t13036, t13042, t13048)
}
