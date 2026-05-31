//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 969/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk969<F: Float>(t16596: F, t25927: F, t2314: F, t32677: F, t4034: F, t5107: F, t652: F, t8326: F, t1437: F, t31: F, t607: F, t8308: F) -> (F, F, F, F, F) {
    let t119763 = t25927 * t16596;
    let t119824 = F::cast_from(2.0_f64) * t2314 * t32677;
    let t119826 = F::cast_from(2.0_f64) * t4034 * t32677;
    let t119830 = F::cast_from(2.0_f64) * t652 * t5107 * t8326;
    let t119878 = t1437 * t31;
    let t119879 = t119878 * t607;
    let t119880 = t8308 * t119879;
    (t119763, t119824, t119826, t119830, t119880)
}
