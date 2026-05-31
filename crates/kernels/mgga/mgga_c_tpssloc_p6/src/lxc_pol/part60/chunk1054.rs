//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1054/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1054<F: Float>(t5456: F, t8828: F, t116075: F, t117477: F, t122988: F, t123001: F, t124803: F, t126091: F, t126103: F, t129084: F, t31860: F, t32338: F, t33669: F, t33677: F, t34122: F, t34132: F, t5389: F, t5392: F, t5441: F, t5445: F, t63: F, t7246: F, t8513: F, t8663: F, t8824: F, t8825: F) -> (F, F) {
    let t130377 = t8828 * t5456;
    let t130412 = -F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t124803 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t123001 * t34122 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t31860 * t8513 * t8824 * t5445 - F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t126091 * t117477 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t33677 * t34132 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8663 * t8513 * t32338 * t5441 - F::cast_from(35.0_f64) / F::cast_from(12.0_f64) * t116075 * t8513 * t8824 * t5389 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t7246 * t8513 * t8824 * t5392 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t122988 * t34122 - F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t33669 * t34132 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8663 * t8513 * t126103 * t63 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t129084 * t8825;
    (t130377, t130412)
}
