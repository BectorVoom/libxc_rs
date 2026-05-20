//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2313/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2313<F: Float>(t40: F, t12908: F, t20749: F, t12923: F, t4194: F, t5398: F, t20800: F, t262: F, t10143: F, t20778: F, t13115: F, t16586: F, t12950: F, t1430: F, t16558: F, t16637: F, t17635: F, t20217: F, t20234: F, t2291: F, t3966: F, t4104: F, t607: F, t67060: F, t75: F, t767: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t67228 = F::new(36.0) * t12908 * t20749;
    let t67230 = t4194 * t12923 * t5398;
    let t67231 = F::new(36.0) * t67230;
    let t67235 = t262 * t20800;
    let t67239 = t20778 * t10143;
    let t67243 = t13115 * t16586;
    let t67244 = F::new(36.0) * t67243;
    let t67262 = piecewise3::<F>(t146, F::new(0.0), -F::new(56.0) / F::new(81.0) * t2291 * t20234 * t607 + F::new(8.0) / F::new(9.0) * t16637 * t3966 + F::new(8.0) / F::new(9.0) * t1430 * t17635 - F::new(2.0) / F::new(3.0) * t12950 * t5398 - F::new(2.0) / F::new(3.0) * t4104 * t16558 - F::new(2.0) / F::new(9.0) * t75 * t20217 * t607 + F::new(2.0) / F::new(3.0) * t767 * t67060);
    (t67228, t67231, t67235, t67239, t67244, t67262)
}
