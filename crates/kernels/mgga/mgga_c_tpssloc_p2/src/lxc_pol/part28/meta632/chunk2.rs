//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1991/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1991<F: Float>(t87753: F, t225: F, t26732: F, t87776: F, t87779: F, t87786: F, t10110: F, t2597: F, t26582: F, t26690: F, t2719: F, t7830: F, t7841: F, t7842: F, t82172: F, t82174: F, t82182: F, t85101: F, t855: F, t866: F, t87047: F, t87050: F, t87746: F, t87765: F, t87773: F, t87784: F, t9593: F) -> F {
    let t92846 = F::cast_from(0.3289868133696452873e-1_f64) * t87753;
    let t92847 = t26732 * t225;
    let t92862 = F::cast_from(0.16449340668482264365e-1_f64) * t87776;
    let t92863 = F::cast_from(0.16449340668482264365e-1_f64) * t87779;
    let t92866 = F::cast_from(0.15352717957250113407e0_f64) * t87786;
    let t92871 = F::cast_from(0.16449340668482264365e-1_f64) * t87047 - F::cast_from(0.46058153871750340222e0_f64) * t87050 - F::cast_from(0.16449340668482264365e-1_f64) * t87746 - t85101 - t92846 - F::new(2.0) * t92847 * t866 - F::new(2.0) * t9593 * t7842 + F::new(4.0) * t9593 * t7830 + F::new(4.0) * t2597 * t26690 + F::new(4.0) * t2597 * t26582 - F::cast_from(0.39478417604357434476e0_f64) * t87765 + F::cast_from(0.16449340668482264365e-1_f64) * t82172 + F::cast_from(0.15352717957250113407e0_f64) * t82174 - F::cast_from(0.16449340668482264365e-1_f64) * t87773 + t92862 + t92863 - F::cast_from(0.16449340668482264365e-1_f64) * t82182 - F::cast_from(0.3289868133696452873e-1_f64) * t87784 - t92866 - F::new(6.0) * t855 * t10110 * t7841 * t2719;
    t92871
}
