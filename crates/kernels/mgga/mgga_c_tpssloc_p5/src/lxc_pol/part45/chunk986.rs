//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 986/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk986<F: Float>(t1888: F, t31333: F, t82159: F, t23012: F, t8548: F, t214: F, t7084: F, t6552: F, t6555: F, t10049: F, t10110: F, t112863: F, t112868: F, t112872: F, t112877: F, t112881: F, t2054: F, t24297: F, t25168: F, t2597: F, t2718: F, t2719: F, t2743: F, t31399: F, t31409: F, t31423: F, t6631: F, t6663: F, t82197: F, t855: F, t8553: F, t8562: F, t8563: F, t865: F, t92981: F, t9590: F) -> (F, F) {
    let t114842 = t1888 * t82159 * t31333;
    let t114864 = t23012 * t8548;
    let t114865 = F::cast_from(0.63969658155208805863e-1_f64) * t114864;
    let t114866 = t214 * t7084;
    let t114868 = t6552 * t114866 * t6555;
    let t114870 = F::cast_from(0.3289868133696452873e-1_f64) * t114842 + F::new(4.0) * t2597 * t31409 + F::new(2.0) * t9590 * t8553 + t112863 - t82197 * t2054 - t112868 + t112872 - F::new(6.0) * t855 * t10110 * t8562 * t2719 - F::new(12.0) * t25168 * t92981 * t6631 - t31423 * t2743 - t10049 * t8563 - F::new(2.0) * t24297 * t6663 + t112877 - t112881 + F::new(4.0) * t855 * t2718 * t31399 * t865 - t114865 - F::cast_from(0.3289868133696452873e-1_f64) * t114868;
    (t114866, t114870)
}
