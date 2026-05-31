//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2243/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2243<F: Float>(t83244: F, t974: F, t985: F, t3030: F, t343: F, t25483: F, t25486: F, t25490: F, t25492: F, t1022: F, t1058: F, t1060: F, t23633: F, t23670: F, t23678: F, t25479: F, t25499: F, t25554: F, t25555: F, t25705: F, t25713: F, t3200: F, t4680: F, t4684: F, t6687: F, t6743: F, t82668: F, t82823: F, t82828: F, t82830: F, t83245: F, t83246: F, t88155: F, t89375: F) -> F {
    let t89498 = t83244 * t974 * t985;
    let t89499 = t343 * t3030;
    let t89501 = t89499 * t25483 * t25486;
    let t89505 = t89499 * t25490 * t25492;
    let t89515 = F::cast_from(0.43864908449286038306e-1_f64) * t23670 * t25479 + F::cast_from(0.54831135561607547884e-2_f64) * t23633 * t6743 * t4680 * t25554 + F::cast_from(2.0_f64) * t1058 * t25705 * t1022 * t1060 + F::cast_from(0.18277045187202515961e-2_f64) * t82823 + F::cast_from(0.54831135561607547884e-2_f64) * t83245 * t83246 * t89375 * t23678 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t88155 * t25713 - F::cast_from(0.3289868133696452873e-1_f64) * t89498 * t89501 + F::cast_from(0.16449340668482264365e-1_f64) * t89498 * t89505 + F::cast_from(0.27415567780803773942e-2_f64) * t82828 + F::cast_from(0.97477574331746751793e-2_f64) * t82830 - F::cast_from(0.14621636149762012769e-1_f64) * t82668 * t25555 - F::cast_from(2.0_f64) * t3200 * t25499 * t4684;
    t89515
}
