//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1985/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1985<F: Float>(t101593: F, t101618: F, t101634: F, t101656: F, t101672: F, t101687: F, t101705: F, t101734: F, t101751: F, t2054: F, t26690: F, t26700: F, t26703: F, t4147: F, t4268: F, t4273: F, t59519: F, t85129: F, t855: F, t858: F, t866: F, t98941: F, t98945: F, t98963: F, t98966: F) -> F {
    let t101761 = -t85129 - F::cast_from(0.15352717957250113407e0_f64) * t98941 - F::cast_from(0.16449340668482264365e-1_f64) * t98945 + F::new(4.0) * t4268 * t26703 + F::new(4.0) * t4147 * t26690 - F::cast_from(0.19739208802178717238e0_f64) * t98963 - t101593 * t866 - F::cast_from(0.16449340668482264365e-1_f64) * t98966 - t855 * t858 * (t101618 + t101634 + t101656 + t101672 + t101687 + t101705 + t101734 + t101751) + F::new(4.0) * t26700 * t4273 - F::new(2.0) * t59519 * t2054;
    t101761
}
