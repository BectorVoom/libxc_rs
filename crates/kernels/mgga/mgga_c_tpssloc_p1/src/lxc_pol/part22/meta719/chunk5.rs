//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2332/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2332<F: Float>(t46657: F, t5593: F, t120: F, t20852: F, t13258: F, t20983: F, t16839: F, t16841: F, t2643: F, t4178: F, t4180: F, t4182: F, t4234: F, t47307: F, t58353: F, t58363: F, t58373: F, t58379: F, t58381: F, t58904: F, t67596: F, t67607: F, t829: F) -> (F, F) {
    let t67612 = t46657 * t5593;
    let t67620 = t120 * t20852;
    let t67625 = t13258 * t20983;
    let t67636 = -t2643 * t4180 * t16839 * t4234 / F::new(1024.0) - t2643 * t4180 * t67607 * t829 / F::new(3072.0) - F::new(7.0) / F::new(192.0) * t67612 - F::new(3.0) / F::new(512.0) * t58904 * t16841 + t47307 * t4180 * t67607 * t67596 / F::new(128.0) + t4178 * t4180 * t67620 * t4182 / F::new(1536.0) + F::new(7.0) / F::new(192.0) * t67625 - t2643 * t4180 * t67620 * t829 / F::new(3072.0) + F::new(7.0) / F::new(256.0) * t58353 + F::new(7.0) / F::new(1536.0) * t58363 - F::new(7.0) / F::new(192.0) * t58373 - F::new(7.0) / F::new(192.0) * t58379 + F::new(7.0) / F::new(768.0) * t58381;
    (t67620, t67636)
}
