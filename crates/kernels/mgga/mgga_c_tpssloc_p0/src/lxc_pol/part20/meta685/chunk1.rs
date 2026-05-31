//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2596/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2596<F: Float>(t3610: F, t52627: F, t11154: F, t11668: F, t11680: F, t11688: F, t11825: F, t11863: F, t1227: F, t15453: F, t15569: F, t1735: F, t3577: F, t3580: F, t44996: F, t4582: F, t48554: F, t4954: F, t4989: F, t5024: F, t52610: F, t52615: F, t52619: F, t52621: F) -> F {
    let t52628 = t3610 * t52627;
    let t52639 = -t52610 - F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t1227 * t4582 * t15453 * t48554 + t52615 * t3580 / F::cast_from(144.0_f64) - t52619 / F::cast_from(2304.0_f64) - t52621 / F::cast_from(1152.0_f64) + t15569 * t11688 / F::cast_from(144.0_f64) - t44996 * t4954 / F::cast_from(1536.0_f64) + t52628 * t11680 / F::cast_from(144.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t3577 * t11668 * t1735 * t11154 + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t11825 * t4989 + t5024 * t11863 / F::cast_from(144.0_f64);
    t52639
}
