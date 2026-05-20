//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3184/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3184<F: Float>(t1227: F, t13969: F, t18589: F, t15743: F, t5005: F, t1174: F, t6177: F, t698: F, t11709: F, t15455: F, t15459: F, t15463: F, t15525: F, t15535: F, t15569: F, t15612: F, t15631: F, t15650: F, t1653: F, t18321: F, t19058: F, t3552: F, t3557: F, t3560: F, t3577: F, t3578: F, t5024: F, t52906: F, t53083: F, t53087: F, t55723: F, t974: F) -> F {
    let t66052 = t1227 * t13969 * t18589;
    let t66054 = t5005 * t15743;
    let t66057 = t1174 * t698 * t6177;
    let t66067 = F::new(5.0) / F::new(486.0) * t5024 * t15455 + t53083 * t15631 / F::new(48.0) - t53087 * t15535 / F::new(288.0) + t11709 * t19058 / F::new(768.0) + t5024 * t15650 / F::new(108.0) + t5024 * t15612 / F::new(216.0) - t3577 * t3578 * t15525 * t1653 / F::new(2304.0) + t15569 * t15459 / F::new(432.0) + t15569 * t15463 / F::new(216.0) - t52906 / F::new(216.0) - t66052 / F::new(864.0) + F::new(5.0) / F::new(5184.0) * t66054 - t66057 / F::new(972.0) + t1174 * t974 * t3560 * t55723 / F::new(108.0) - F::new(11.0) / F::new(324.0) * t18321 * t3552 - F::new(11.0) / F::new(162.0) * t18321 * t3557;
    t66067
}
