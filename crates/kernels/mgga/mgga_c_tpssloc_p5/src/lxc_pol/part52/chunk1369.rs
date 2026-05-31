//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1369/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1369<F: Float>(t12571: F, t31863: F, t116114: F, t39063: F, t45844: F, t8662: F, t33676: F, t9239: F, t116082: F, t116111: F, t116115: F, t116119: F, t116124: F, t119913: F, t119938: F, t119944: F, t119952: F, t121024: F, t121032: F, t121074: F, t121081: F, t121087: F, t31677: F, t31684: F, t31693: F, t31857: F, t31860: F, t31868: F, t33564: F, t33568: F, t33572: F, t33669: F, t33677: F, t8663: F) -> F {
    let t122976 = t12571 * t31863;
    let t122979 = t39063 * t116114;
    let t122988 = t45844 * t8662;
    let t123001 = t9239 * t33676;
    let t123020 = -F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t122976 * t31684 + F::cast_from(35.0_f64) / F::cast_from(24.0_f64) * t122979 * t121024 - F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t116115 * t121032 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t116111 * t33568 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t116119 * t33568 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t122988 * t31677 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t33669 * t31693 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t116124 * t33564 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t116082 * t33564 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t31860 * t119913 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t31860 * t121074 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t123001 * t31677 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t33677 * t31693 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t31857 * t33572 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t31868 * t33572 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t8663 * t121081 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t8663 * t119952 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t8663 * t121087 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t31860 * t119938 + F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t8663 * t119944;
    t123020
}
