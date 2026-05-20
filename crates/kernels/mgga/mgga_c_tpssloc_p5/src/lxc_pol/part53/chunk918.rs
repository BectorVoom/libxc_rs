//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 918/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk918<F: Float>(t2053: F, t2718: F, t7841: F, t1492: F, t8728: F, t31976: F, t31978: F, t31982: F, t32835: F, t32838: F, t32841: F, t32845: F, t32847: F) -> (F, F, F) {
    let t33935 = t2718 * t2053 * t7841;
    let t33940 = t1492 * t8728;
    let t33947 = -t31976 - F::cast_from(0.19378922925187387609e-1_f64) * t32835 - t31978 - F::cast_from(0.32298204875312312682e-2_f64) * t32838 + t32841 / F::new(384.0) - t32845 / F::new(384.0) - t31982 - t32847 / F::new(96.0);
    (t33935, t33940, t33947)
}
