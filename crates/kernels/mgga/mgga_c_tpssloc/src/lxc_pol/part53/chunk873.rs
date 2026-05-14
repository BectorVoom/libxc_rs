//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 873/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk873<F: Float>(t1888: F, t33457: F, t82159: F, t1880: F, t214: F, t225: F, t258: F, t26653: F, t6562: F, t8547: F, t86893: F, t7823: F, t6552: F, t6555: F, t25224: F, t31419: F) -> (F, F, F, F, F, F) {
    let t121382 = t1888 * t82159 * t33457;
    let t121391 = t1880 * t214 * t26653 * t225 * t258;
    let t121399 = t6562 * t86893 * t8547;
    let t121401 = t214 * t7823;
    let t121403 = t6552 * t121401 * t6555;
    let t121409 = t1880 * t25224 * t31419;
    (t121382, t121391, t121399, t121401, t121403, t121409)
}
