//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2991/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2991<F: Float>(t10422: F, t17676: F, t3070: F, t225: F, t61618: F, t10413: F, t14122: F, t14126: F, t14489: F, t1539: F, t1616: F, t2979: F, t3071: F, t369: F, t378: F, t4343: F, t4650: F, t49929: F, t49934: F, t50132: F, t50147: F, t50169: F, t50172: F, t50174: F, t50181: F, t59715: F, t59767: F, t61871: F, t68: F, t973: F, t977: F) -> (F, F) {
    let t62602 = t3070 * t10422 * t17676;
    let t62604 = t61618 * t225;
    let t62616 = t50132 / F::cast_from(432.0_f64) + t49929 * t14122 / F::cast_from(1152.0_f64) - t50147 / F::cast_from(5184.0_f64) - t49934 * t14126 / F::cast_from(2304.0_f64) - t10413 * t3071 * t1616 * t61871 / F::cast_from(1152.0_f64) - t3070 * t3071 * t4650 * t4343 / F::cast_from(576.0_f64) - t973 * t977 * t59767 / F::cast_from(72.0_f64) - t973 * t2979 * t59715 / F::cast_from(36.0_f64) - t50169 / F::cast_from(5184.0_f64) + t62602 / F::cast_from(1728.0_f64) + t62604 * t68 * t369 * t378 / F::cast_from(3072.0_f64) - t50172 / F::cast_from(81.0_f64) + t3070 * t3071 * t14489 * t1539 / F::cast_from(2304.0_f64) - t50174 / F::cast_from(3456.0_f64) + t50181 / F::cast_from(5184.0_f64);
    (t62604, t62616)
}
