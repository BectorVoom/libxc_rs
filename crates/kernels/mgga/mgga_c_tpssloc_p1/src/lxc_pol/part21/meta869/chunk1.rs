//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3183/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3183<F: Float>(t1227: F, t13969: F, t18345: F, t1174: F, t1177: F, t18943: F, t3536: F, t3555: F, t52872: F, t52875: F, t55723: F, t63294: F, t63298: F, t63302: F, t65992: F, t65994: F, t65996: F, t65998: F, t66001: F, t66015: F, t66024: F, t974: F) -> F {
    let t66027 = t1227 * t13969 * t18345;
    let t66029 = -t65992 / F::new(216.0) - t65994 / F::new(216.0) + t65996 / F::new(1152.0) + t65998 / F::new(1152.0) - t66001 / F::new(216.0) - t1174 * t1177 * t63294 / F::new(72.0) - t1174 * t1177 * t63298 / F::new(144.0) - t1174 * t1177 * t63302 / F::new(48.0) + t3536 * t18943 / F::new(1536.0) + t66015 / F::new(648.0) - t1174 * t974 * t3555 * t55723 / F::new(72.0) + t52872 / F::new(5184.0) - t52875 / F::new(1728.0) + F::new(5.0) / F::new(5184.0) * t66024 + F::new(5.0) / F::new(1728.0) * t66027;
    t66029
}
