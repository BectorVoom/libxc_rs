//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1287/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1287<F: Float>(t225: F, t34254: F, t8872: F, t94490: F, t24574: F, t34244: F, t118084: F, t1252: F, t125510: F, t15797: F, t1761: F, t24589: F, t24615: F, t27388: F, t27406: F, t27742: F, t27746: F, t27747: F, t27751: F, t27760: F, t27784: F, t27785: F, t32510: F, t32516: F, t32519: F, t34306: F, t34318: F, t34331: F, t34338: F, t3487: F, t3593: F, t466: F, t498: F, t7283: F, t7300: F, t7351: F, t86403: F, t8888: F) -> F {
    let t125713 = t34254 * t225;
    let t125729 = t94490 * t8872;
    let t125732 = t24574 * t34244;
    let t125752 = -t125713 * t1252 - F::new(0.54831135561607547883e-2) * t7283 * t32519 * t27388 + F::new(0.3289868133696452873e-1) * t7283 * t27751 * t32510 + F::new(4.0) * t7351 * t27747 - F::new(12.0) * t27784 * t27785 * t27760 - t3593 * t34306 + F::new(2.0) * t15797 * t8888 + F::new(0.14621636149762012769e-1) * t125729 - t118084 * t1761 - F::new(0.54831135561607547883e-2) * t125732 - F::new(2.0) * t7351 * t27742 + t466 * t125510 * t498 - F::new(0.54831135561607547883e-2) * t24589 * t86403 * t34338 - F::new(0.43864908449286038307e-1) * t27406 * t32516 + F::new(0.3289868133696452873e-1) * t7283 * t7300 * t24615 * t27746 - t3487 * t34306 - F::new(6.0) * t3593 * t34331 + F::new(2.0) * t3593 * t34318;
    t125752
}
