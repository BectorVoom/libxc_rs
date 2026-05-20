//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1017/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1017<F: Float>(t118586: F, t118588: F, t118596: F, t118602: F, t112829: F, t114724: F, t114725: F, t114736: F, t116608: F, t116610: F, t116613: F, t116615: F, t118590: F, t118592: F, t118594: F, t118606: F, t118608: F, t118610: F, t118612: F) -> F {
    let t123571 = F::cast_from(0.5383034145885385447e-3_f64) * t118586;
    let t123572 = F::new(7.0) / F::new(144.0) * t118588;
    let t123576 = F::new(7.0) / F::new(576.0) * t118596;
    let t123578 = F::new(7.0) / F::new(576.0) * t118602;
    let t123583 = t123571 + t123572 - t118590 / F::new(96.0) - t118592 / F::new(96.0) - t118594 / F::new(96.0) + t123576 + t114724 + t114725 + F::cast_from(0.22608743412718618877e-1_f64) * t112829 - t123578 + t116608 - t116610 - F::cast_from(0.19378922925187387609e-1_f64) * t118606 - t118608 / F::new(384.0) + t118610 / F::new(96.0) + t118612 / F::new(96.0) - t114736 + t116613 + t116615;
    t123583
}
