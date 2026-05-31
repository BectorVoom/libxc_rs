//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2247/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2247<F: Float>(t25751: F, t82431: F, t4657: F, t6703: F, t7554: F, t82573: F, t1920: F, t2966: F, t7561: F, t225: F, t25789: F, t1066: F, t13742: F, t1635: F, t1956: F, t23346: F, t23394: F, t23588: F, t25407: F, t25732: F, t3169: F, t4542: F, t50653: F, t50690: F, t6687: F, t6704: F, t6706: F, t82402: F, t83398: F, t83408: F) -> F {
    let t89597 = F::cast_from(0.18277045187202515961e-2_f64) * t82431 * t25751;
    let t89598 = t6703 * t4657;
    let t89609 = t82573 * t7554;
    let t89617 = t1920 * t2966 * t7561;
    let t89620 = t25789 * t225;
    let t89623 = -F::cast_from(2.0_f64) * t50653 * t1956 - F::cast_from(0.27415567780803773942e-2_f64) * t83398 + F::cast_from(0.14621636149762012769e-1_f64) * t82402 * t25751 - t89597 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t89598 * t6706 - F::cast_from(2.0_f64) * t3169 * t25732 + F::cast_from(0.3289868133696452873e-1_f64) * t6687 * t6704 * t23394 * t13742 - t83408 * t1635 - F::cast_from(0.48738787165873375897e-2_f64) * t89609 + F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25407 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t4542 * t23588 - F::cast_from(0.18277045187202515961e-2_f64) * t89617 - t50690 * t1956 - F::cast_from(2.0_f64) * t89620 * t1066;
    t89623
}
