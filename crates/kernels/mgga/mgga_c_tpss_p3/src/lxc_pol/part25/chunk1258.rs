//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1258/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1258<F: Float>(t33: F, t259: F, t479: F, t21701: F, t1289: F, t1826: F, t21741: F, t4579: F, t57: F, t6393: F, t21709: F, t1791: F, t21165: F, t1675: F, t1792: F, t18648: F, t18666: F, t19349: F, t20246: F, t20255: F, t20257: F, t20264: F, t20276: F, t20278: F, t21116: F, t21123: F, t21129: F, t21133: F, t21136: F, t21139: F, t21146: F, t5785: F, t6073: F, t6077: F, t6080: F, t6304: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F, F, F) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t21742 = piecewise3::<F>(t480, F::cast_from(0.0_f64), t21701);
    let t21749 = piecewise3::<F>(t386, t21741, t21742 * t57 / F::cast_from(2.0_f64) - t6393 * t1289 - t1826 * t4579 / F::cast_from(2.0_f64));
    let t21750 = t21709 + t21749;
    let t21756 = t1791 * t21165;
    let t21784 = F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t20257 + t18648 + t1675 * t21756 / F::cast_from(3.0_f64) + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t19349 * t20264 + F::cast_from(10.0_f64) * t18666 * t21116 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t20276 + t21146 * t1792 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6073 * t6304 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t20278 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t20255 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t20246 * t6077 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t21123 * t1792 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t5785 * t21129 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t5785 * t21133 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t21136 * t1792 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t21139 * t1792 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t6080 * t6304;
    (t21742, t21750, t21756, t21784)
}
