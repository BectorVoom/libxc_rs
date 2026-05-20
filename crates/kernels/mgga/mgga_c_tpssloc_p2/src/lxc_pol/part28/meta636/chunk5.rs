//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2025/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2025<F: Float>(t91394: F, t91398: F, t91402: F, t91404: F, t80920: F, t80922: F, t80940: F, t80943: F, t80959: F, t80989: F, t80992: F, t80998: F, t81007: F, t84555: F, t84558: F, t91400: F, t91413: F, t91416: F) -> F {
    let t93757 = F::new(119.0) / F::new(3456.0) * t91394;
    let t93760 = F::new(35.0) / F::new(108.0) * t91398;
    let t93762 = F::new(7.0) / F::new(36.0) * t91402;
    let t93763 = F::cast_from(0.33913115119077928316e-1_f64) * t91404;
    let t93773 = -t93757 + F::cast_from(0.28260929265898273597e-2_f64) * t80920 + F::cast_from(0.28260929265898273597e-2_f64) * t80922 - t93760 - F::cast_from(0.13565246047631171326e0_f64) * t91400 + t93762 + t93763 - F::cast_from(0.45217486825437237756e-1_f64) * t80940 - F::cast_from(0.56521858531796547194e-2_f64) * t80943 - t84555 - F::cast_from(0.33913115119077928316e-1_f64) * t80959 + t84558 + F::new(7.0) / F::new(1152.0) * t80989 + F::new(7.0) / F::new(576.0) * t80992 - F::new(7.0) / F::new(576.0) * t80998 + F::new(7.0) / F::new(1152.0) * t81007 + t91413 / F::new(96.0) + t91416 / F::new(768.0);
    t93773
}
