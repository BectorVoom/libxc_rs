//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 904/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk904<F: Float>(t40889: F, t8733: F, t114945: F, t114965: F, t116654: F, t116709: F, t121745: F, t121749: F, t121753: F, t13463: F, t1528: F, t2054: F, t25168: F, t2597: F, t26700: F, t26713: F, t2713: F, t31999: F, t32002: F, t33974: F, t4147: F, t4268: F, t4272: F, t7107: F, t8734: F, t92439: F) -> (F,) {
    let t123699 = t40889 * t8733;
    let t123711 = -t4268 * t31999 - 2.0 * t26700 * t7107 + t116654 - t2713 * t33974 - 2.0 * t26713 * t7107 + 0.76763589786250567037e-1 * t114945 + 2.0 * t13463 * t8734 - 0.19739208802178717238e0 * t121745 - 0.16449340668482264365e-1 * t121749 + 24.0 * t25168 * t123699 * t4272 + 4.0 * t4147 * t32002 + 0.16449340668482264365e-1 * t121753 - t2597 * t33974 - 2.0 * t92439 * t2054 + 0.16449340668482264365e-1 * t114965 - t116709 * t1528;
    (t123711,)
}
