//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1973/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1973<F: Float>(t86950: F, t86955: F, t86967: F, t225: F, t26708: F, t86991: F, t13065: F, t2054: F, t24325: F, t24330: F, t26679: F, t2718: F, t4147: F, t4268: F, t47609: F, t7092: F, t82108: F, t82115: F, t82120: F, t85060: F, t855: F, t865: F, t86997: F) -> (F, F, F, F, F) {
    let t92431 = F::cast_from(0.15352717957250113407e0_f64) * t86950;
    let t92432 = F::cast_from(0.12793931631041761173e0_f64) * t86955;
    let t92434 = F::cast_from(0.15352717957250113407e0_f64) * t86967;
    let t92439 = t26708 * t225;
    let t92458 = F::cast_from(0.12793931631041761173e0_f64) * t86991;
    let t92464 = -F::new(2.0) * t47609 * t2054 + F::new(2.0) * t4147 * t24330 + F::new(4.0) * t855 * t2718 * t26679 * t865 - F::cast_from(0.49348022005446793095e-1_f64) * t82108 + F::new(4.0) * t13065 * t7092 - t92458 + F::new(4.0) * t4268 * t24325 - F::cast_from(0.15352717957250113407e0_f64) * t82115 + F::cast_from(0.6579736267392905746e-1_f64) * t82120 - t85060 - F::cast_from(0.16449340668482264365e-1_f64) * t86997;
    (t92431, t92432, t92434, t92439, t92464)
}
