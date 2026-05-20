//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1980/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1980<F: Float>(t87291: F, t87293: F, t87300: F, t87304: F, t87308: F, t81857: F, t81859: F, t81874: F, t81877: F, t81883: F, t87287: F, t87289: F, t87296: F, t87298: F, t87306: F, t87312: F, t87316: F, t87322: F) -> F {
    let t92626 = F::new(7.0) / F::new(36.0) * t87291;
    let t92627 = F::cast_from(0.33913115119077928316e-1_f64) * t87293;
    let t92630 = F::new(35.0) / F::new(144.0) * t87300;
    let t92633 = F::new(35.0) / F::new(108.0) * t87304;
    let t92635 = F::cast_from(0.33913115119077928316e-1_f64) * t87308;
    let t92642 = -F::new(5.0) / F::new(96.0) * t87287 + t87289 / F::new(96.0) + t92626 + t92627 - t87296 / F::new(384.0) - t87298 / F::new(768.0) - t92630 - F::new(35.0) / F::new(288.0) * t81857 + F::cast_from(0.28260929265898273597e-2_f64) * t81859 - t92633 - F::cast_from(0.13565246047631171326e0_f64) * t87306 - t92635 + F::cast_from(0.80745512188280781706e-3_f64) * t87312 + F::cast_from(0.48447307312968469024e-2_f64) * t87316 + F::cast_from(0.67287926823567318088e-4_f64) * t81874 + F::cast_from(0.67287926823567318088e-4_f64) * t81877 - F::cast_from(0.21083550404717759668e-2_f64) * t81883 - t87322 / F::new(96.0);
    t92642
}
