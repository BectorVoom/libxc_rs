//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1656/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1656<F: Float>(t225: F, t7085: F, t10110: F, t2053: F, t2719: F, t23251: F, t23261: F, t7106: F, t865: F, t2718: F, t2742: F, t10049: F, t2054: F, t23243: F, t23249: F, t23254: F, t23259: F, t23266: F, t23274: F, t2597: F, t2713: F, t2743: F, t7087: F, t7092: F, t7107: F, t855: F, t866: F, t9590: F, t9593: F) -> (F, F, F, F, F, F, F) {
    let t24305 = t7085 * t225;
    let t24314 = t10110 * t2053 * t2719;
    let t24318 = F::cast_from(0.52089578783527170489e-1_f64) * t23251;
    let t24321 = F::cast_from(0.12793931631041761173e0_f64) * t23261;
    let t24324 = t7106 * t865;
    let t24325 = t2718 * t24324;
    let t24330 = t2718 * t2053 * t2742;
    let t24333 = -F::new(2.0) * t2597 * t7107 + F::cast_from(0.9869604401089358619e-1_f64) * t23243 - t9590 * t2054 - F::new(2.0) * t24305 * t866 - t10049 * t2054 - F::new(2.0) * t9593 * t2054 + F::new(4.0) * t2713 * t7092 - F::new(6.0) * t855 * t24314 - F::cast_from(0.76763589786250567036e-1_f64) * t23249 + t24318 - F::cast_from(0.16449340668482264365e-1_f64) * t23254 + F::cast_from(0.16449340668482264365e-1_f64) * t23259 + t24321 - F::cast_from(0.3289868133696452873e-1_f64) * t23266 - t7087 * t2743 + F::new(4.0) * t855 * t24325 + F::cast_from(0.6579736267392905746e-1_f64) * t23274 + F::new(2.0) * t855 * t24330;
    (t24305, t24314, t24318, t24321, t24325, t24330, t24333)
}
