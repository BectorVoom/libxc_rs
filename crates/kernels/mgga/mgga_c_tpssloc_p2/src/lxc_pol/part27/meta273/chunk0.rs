//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1306/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1306<F: Float>(t533: F, t7752: F, t1390: F, t1983: F, t2019: F, t5161: F, t113: F, t1442: F, t1459: F, t1774: F, t1849: F, t1869: F, t1976: F, t1980: F, t510: F, t574: F, t6517: F, t652: F, t7451: F, t7457: F, t7460: F, t7463: F, t7470: F, t7472: F, t7670: F, t7681: F, t7686: F, t7690: F) -> (F, F, F, F) {
    let t7753 = t533 * t7752;
    let t7754 = t7753 * t1390;
    let t7755 = t1983 * t7754;
    let t7756 = t2019 * t5161;
    let t7757 = t1983 * t7756;
    let t7758 = -t113 * t7670 - t1442 * t1976 - F::cast_from(2.0_f64) * t1459 * t6517 - t1774 * t1869 + t1849 * t1980 - t510 * t7451 + t574 * t7681 - F::cast_from(2.0_f64) * t652 * t7472 - t7457 - t7460 - t7463 - t7470 + t7686 + t7690 + t7755 - t7757;
    (t7753, t7754, t7756, t7758)
}
