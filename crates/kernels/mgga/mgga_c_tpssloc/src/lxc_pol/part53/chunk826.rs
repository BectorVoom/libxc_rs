//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 826/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk826<F: Float>(t26611: F, t26678: F, t858: F, t25036: F, t25042: F, t25047: F, t25056: F, t25061: F, t2597: F, t26582: F, t26591: F, t2713: F, t4147: F, t4268: F, t4273: F, t7087: F, t7092: F, t7107: F, t7830: F, t855: F) -> (F, F, F) {
    let t26679 = t26611 + t26678;
    let t26680 = t858 * t26679;
    let t26684 = -F::new(0.82246703342411321825e-2) * t25036 + F::new(2.0) * t855 * t26582 + F::new(2.0) * t2713 * t7830 + F::new(2.0) * t4268 * t7092 + F::new(0.9869604401089358619e-1) * t25042 + F::new(0.3289868133696452873e-1) * t25047 - t26591 + F::new(2.0) * t2597 * t7830 + F::new(0.3289868133696452873e-1) * t25056 + F::new(2.0) * t7087 * t4273 - t855 * t26680 - t4147 * t7107 + F::new(0.16449340668482264365e-1) * t25061;
    (t26679, t26680, t26684)
}
