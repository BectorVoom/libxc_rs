//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1362/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1362<F: Float>(t265: F, t394: F, t100497: F, t105863: F, t105890: F, t105934: F, t105971: F, t106430: F, t106460: F, t106492: F, t106526: F, t106606: F, t1070: F, t1637: F, t193: F, t21376: F, t21697: F, t23742: F, t25840: F, t336: F, t4700: F, t5946: F, t5950: F, t6822: F, t83479: F, t89702: F) -> F {
    let t395 = t265 < t394;
    let t106607 = piecewise3::<F>(t395, t193 * t336 * (t105863 + t105890 + t105934 + t105971 + t106430 + t106460 + t106492 + t106526) * t1070 - F::new(3.0) * t4700 * t100497 * t1637 + F::new(6.0) * t4700 * t89702 * t5950 - F::new(3.0) * t4700 * t25840 * t5946 - F::new(6.0) * t4700 * t83479 * t21376 + F::new(6.0) * t4700 * t23742 * t1637 * t5946 - t4700 * t6822 * t21697, t106606);
    t106607
}
