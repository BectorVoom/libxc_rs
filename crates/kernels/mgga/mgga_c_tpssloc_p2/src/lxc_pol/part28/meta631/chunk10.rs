//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1987/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1987<F: Float>(t87612: F, t87618: F, t87653: F, t13263: F, t13336: F, t13397: F, t2051: F, t2633: F, t26656: F, t2684: F, t4281: F, t4291: F, t81697: F, t81704: F, t87615: F, t87627: F, t87630: F, t87633: F, t87635: F, t87640: F, t87645: F, t87650: F) -> F {
    let t92760 = F::cast_from(0.3289868133696452873e-1_f64) * t87612;
    let t92768 = F::cast_from(0.3289868133696452873e-1_f64) * t87618;
    let t92781 = F::cast_from(0.16449340668482264365e-1_f64) * t87653;
    let t92782 = -t92760 + F::cast_from(0.9869604401089358619e-1_f64) * t87615 - F::new(6.0) * t13397 * t26656 * t13263 + F::new(6.0) * t4281 * t26656 * t2633 + t92768 - t4291 * t26656 * t2684 + t13336 * t2051 + F::cast_from(0.38381794893125283518e-1_f64) * t81697 - F::cast_from(0.16449340668482264365e-1_f64) * t87627 - F::cast_from(0.9869604401089358619e-1_f64) * t87630 + F::cast_from(0.3289868133696452873e-1_f64) * t87633 - F::cast_from(0.25587863262083522346e0_f64) * t87635 + F::cast_from(0.38381794893125283518e-1_f64) * t81704 + F::cast_from(0.9869604401089358619e-1_f64) * t87640 - F::cast_from(0.39478417604357434476e0_f64) * t87645 - F::cast_from(0.3289868133696452873e-1_f64) * t87650 - t92781;
    t92782
}
