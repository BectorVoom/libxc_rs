//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1263/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1263<F: Float>(t1409: F, t20217: F, t10403: F, t10408: F, t1041: F, t13995: F, t14164: F, t14187: F, t1539: F, t1616: F, t21396: F, t21403: F, t21512: F, t21520: F, t21526: F, t21551: F, t3039: F, t3070: F, t3071: F, t42483: F, t43361: F, t4582: F, t4588: F, t4644: F, t49929: F, t5677: F, t5681: F, t5867: F, t5873: F, t62284: F, t70391: F, t70535: F, t70554: F, t70573: F, t70597: F, t77606: F) -> (F, F) {
    let t77621 = t20217 * t1409;
    let t77637 = 5.0 / 1152.0 * t10403 * t10408 * t5873 * t5677 - t43361 * t3071 * t21396 * t1539 / 192.0 + t49929 * t21526 / 192.0 - t13995 * t21520 / 192.0 - t3070 * t3071 * t5681 * t5867 / 384.0 + t1041 * t4582 * t14164 * t77606 / 128.0 + t70535 / 288.0 + t70554 / 384.0 - t3039 * t4582 * t70391 * t1616 / 768.0 + 5.0 / 1728.0 * t70573 - t62284 / 1728.0 + 5.0 / 1152.0 * t4644 * t21512 + 5.0 / 3456.0 * t1041 * t4582 * t4588 * t77621 + 5.0 / 864.0 * t1041 * t4582 * t14187 * t77606 - t4644 * t21551 / 192.0 - t70597 / 384.0 + t42483 * t3071 * t21403 * t1539 / 1152.0;
    (t77621, t77637)
}
