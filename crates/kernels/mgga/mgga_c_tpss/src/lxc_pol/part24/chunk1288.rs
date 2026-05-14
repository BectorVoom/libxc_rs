//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1288/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1288<F: Float>(t13463: F, t13546: F, t1753: F, t544: F, t5514: F, t626: F, t646: F, t69006: F, t69012: F, t69016: F, t69018: F, t69020: F, t69022: F, t69025: F, t69028: F, t69030: F, t69060: F, t69367: F, t69372: F, t69373: F, t69375: F, t69377: F, t69379: F, t69382: F, t69385: F) -> (F,) {
    let t69386 = -t69006 - 2.0 * t5514 * t13463 - 2.0 * t626 * t1753 * t13546 - 2.0 * t69012 * t646 - t69016 - t69018 - t69020 - t69022 - t69025 - t69028 - t69030 + (t69060 + t69367) * t544 + t69372 - t69373 - t69375 - t69377 - t69379 - t69382 - t69385;
    (t69386,)
}
