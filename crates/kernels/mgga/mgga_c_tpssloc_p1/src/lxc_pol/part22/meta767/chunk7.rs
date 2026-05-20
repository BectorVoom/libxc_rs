//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2599/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2599<F: Float>(t13969: F, t22270: F, t3506: F, t11678: F, t1227: F, t15591: F, t18301: F, t18594: F, t18955: F, t18959: F, t19051: F, t22280: F, t3577: F, t3578: F, t45030: F, t45162: F, t4582: F, t4733: F, t4974: F, t5005: F, t5012: F, t5024: F, t52600: F, t52601: F, t52610: F, t5975: F, t6221: F, t6225: F, t70330: F, t72445: F) -> F {
    let t72470 = t3506 * t13969 * t22270;
    let t72484 = -t52600 + t15591 * t6221 / F::new(1024.0) - F::new(3.0) / F::new(256.0) * t45030 * t4582 * t72445 * t18301 - t19051 * t4974 / F::new(768.0) - F::new(5.0) / F::new(432.0) * t1227 * t4582 * t52601 * t70330 - F::new(5.0) / F::new(1728.0) * t5005 * t18955 - t52610 + t5024 * t18959 / F::new(144.0) + t72470 / F::new(768.0) - t3577 * t3578 * t5012 * t5975 / F::new(768.0) - t45162 * t22280 / F::new(768.0) - t11678 * t3578 * t6225 * t4733 / F::new(768.0) + t5024 * t18594 / F::new(48.0);
    t72484
}
