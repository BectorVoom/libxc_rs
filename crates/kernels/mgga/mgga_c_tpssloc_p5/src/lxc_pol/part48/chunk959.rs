//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 959/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk959<F: Float>(t6562: F, t82133: F, t8547: F, t7106: F, t857: F, t22986: F, t23270: F, t776: F, t112723: F, t112727: F, t112730: F, t112733: F, t112742: F, t112744: F, t114772: F, t114781: F, t114785: F, t114792: F, t23214: F, t25168: F, t2713: F, t2718: F, t31409: F, t31416: F, t6662: F, t855: F, t8553: F, t866: F, t87013: F, t92394: F, t9593: F) -> F {
    let t114795 = t6562 * t82133 * t8547;
    let t114797 = t857 * t7106;
    let t114800 = t22986 * t23270 * t114797 * t776;
    let t114802 = t112723 + F::new(24.0) * t25168 * t92394 * t23214 + F::cast_from(0.3289868133696452873e-1_f64) * t114772 - F::new(12.0) * t87013 * t31416 + F::new(4.0) * t855 * t2718 * t7106 * t6662 + t112727 - t112730 + t112733 - F::cast_from(0.82246703342411321825e-2_f64) * t114781 + F::new(4.0) * t9593 * t8553 - F::new(2.0) * t114785 * t866 + F::new(4.0) * t2713 * t31409 + t112742 + t112744 + F::cast_from(0.82246703342411321824e-2_f64) * t114792 + F::cast_from(0.82246703342411321824e-2_f64) * t114795 + F::cast_from(0.3289868133696452873e-1_f64) * t114800;
    t114802
}
